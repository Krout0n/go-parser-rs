use std::collections::HashMap;
mod parse_util;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0},
    combinator::opt,
    sequence::tuple,
};
use nom::{
    bytes::streaming::take_while,
    character::complete::{char, space1},
    sequence::delimited,
};
use nom::{character::complete::alpha1, IResult};

use maplit::hashmap;
use parse_util::{identifier, reserved};

#[derive(Debug, PartialEq)]
pub enum GoType {
    Int,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Complex64,
    Complex128,
    Byte,
    Rune,
    String,
}

impl From<&str> for GoType {
    fn from(s: &str) -> Self {
        match s {
            "int" => GoType::Int,
            "uint8" => GoType::Uint8,
            "uint16" => GoType::Uint16,
            "uint32" => GoType::Uint32,
            "uint64" => GoType::Uint64,
            "int8" => GoType::Int8,
            "int16" => GoType::Int16,
            "int32" => GoType::Int32,
            "int64" => GoType::Int64,
            "float32" => GoType::Float32,
            "float64" => GoType::Float64,
            "complex64" => GoType::Complex64,
            "complex128" => GoType::Complex128,
            "byte" => GoType::Byte,
            "rune" => GoType::Rune,
            "string" => GoType::String,
            _ => unreachable!("{}", s),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ArgTypes<'a>(HashMap<&'a str, GoType>);

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub args: ArgTypes<'a>,
    pub ret: GoType,
}

#[derive(Debug, PartialEq)]
pub enum TopLevel<'a> {
    Pkg(&'a str),
    Import(Vec<ImportDeclaration<'a>>),
    Function(Function<'a>),
}

#[derive(Debug, PartialEq)]
pub struct ImportDeclaration<'a> {
    pkg_name_opt: Option<&'a str>,
    path: &'a str,
}

impl<'a> ImportDeclaration<'a> {
    fn new(pkg_name_opt: Option<&'a str>, path: &'a str) -> Self {
        Self { pkg_name_opt, path }
    }
}

// PackageClause  = "package" PackageName .
// PackageName    = identifier .
pub fn parse_package_clause(s: &str) -> IResult<&str, TopLevel> {
    let (s, _) = tag("package")(s)?;
    let (s, _) = space1(s)?;
    let (s, pkg_name) = alpha1(s)?;
    Ok((s, TopLevel::Pkg(pkg_name)))
}

// ImportDecl       = "import" ( ImportSpec | "(" { ImportSpec ";" } ")" ) .
// ImportSpec       = [ "." | PackageName ] ImportPath .
// ImportPath       = string_lit .
pub fn parse_import_decl(s: &str) -> IResult<&str, TopLevel> {
    let (s, _) = reserved("import")(s)?;
    // TODO: multiple '(' packages ')'
    let mut parser = opt(identifier);
    let (s, _pkg_name_opt) = parser(s)?;
    // Take the left value for package name.
    let pkg_name_opt = _pkg_name_opt.map(|s| s);
    let (s, pkg_path) = parse_string_literal(s)?;
    Ok((
        s,
        TopLevel::Import(vec![ImportDeclaration::new(pkg_name_opt, pkg_path)]),
    ))
}

fn parse_go_type(s: &str) -> IResult<&str, GoType> {
    let (s, typ) = identifier(s)?;
    Ok((s, GoType::from(typ)))
}

// Thanks to drumato!
// https://github.com/Drumato/peachili/blob/codegen/src/compiler/common/frontend/pass/parser/primitive.rs#L14
fn parse_string_literal(i: &str) -> nom::IResult<&str, &str> {
    let (rest, contents) = delimited(char('"'), take_while(|b: char| b != '"'), char('"'))(i)?;
    Ok((rest, contents))
}

// FunctionDecl = "func" FunctionName Signature [ FunctionBody ] .
// FunctionBody = . // TODO: Implement block.
fn parse_function_decl<'a>(s: &'a str) -> IResult<&'a str, Function<'a>> {
    let (s, _) = reserved("func")(s)?;
    let (s, name) = identifier(s)?;
    let (s, args) = parse_parameters(s)?;
    let ret = GoType::Int;
    Ok((s, Function { name, args, ret }))
}

// Parameters     = "(" [ ParameterList [ "," ] ] ")" .
// ParameterList  = ParameterDecl { "," ParameterDecl } .
// ParameterDecl  = [ IdentifierList ] [ "..." ] Type .
fn parse_parameters(s: &str) -> IResult<&str, ArgTypes> {
    // (x int)
    // ()
    // TODO: (x, y int)
    // TODO: (x int, y string)
    let (s, _) = space0(s)?;
    let parser = tuple((alphanumeric1, space1, parse_go_type));
    let (s, arg_types_opt) = delimited(char('('), opt(parser), char(')'))(s)?;
    let mut m = HashMap::new();
    arg_types_opt.map(|(name, _, typ)| m.insert(name, typ));
    Ok((s, ArgTypes(m)))
}

#[test]
fn test_pkg_stmt() {
    assert_eq!(
        parse_package_clause("package main"),
        Ok(("", TopLevel::Pkg("main")))
    );

    assert_eq!(
        parse_package_clause("package main\n"),
        Ok(("\n", TopLevel::Pkg("main")))
    );

    assert!(parse_package_clause("packagemain").is_err())
}

#[test]
fn test_import_decl() {
    // import   "lib/math"
    assert_eq!(
        parse_import_decl("import \"lib/math\""),
        Ok((
            "",
            TopLevel::Import(vec![ImportDeclaration::new(None, "lib/math")])
        ))
    );

    // import m "lib/math"
    assert_eq!(
        parse_import_decl("import m \"lib/math\""),
        Ok((
            "",
            TopLevel::Import(vec![ImportDeclaration::new(Some("m"), "lib/math")])
        ))
    );

    // import . "lib/math"
}

#[test]
fn test_go_type() {
    assert_eq!(parse_go_type("int"), Ok(("", GoType::Int)));
    assert_eq!(parse_go_type("uint8"), Ok(("", GoType::Uint8)));
    assert_eq!(parse_go_type("uint16"), Ok(("", GoType::Uint16)));
    assert_eq!(parse_go_type("uint32"), Ok(("", GoType::Uint32)));
    assert_eq!(parse_go_type("uint64"), Ok(("", GoType::Uint64)));
    assert_eq!(parse_go_type("int8"), Ok(("", GoType::Int8)));
    assert_eq!(parse_go_type("int16"), Ok(("", GoType::Int16)));
    assert_eq!(parse_go_type("int32"), Ok(("", GoType::Int32)));
    assert_eq!(parse_go_type("int64"), Ok(("", GoType::Int64)));
    assert_eq!(parse_go_type("float32"), Ok(("", GoType::Float32)));
    assert_eq!(parse_go_type("float64"), Ok(("", GoType::Float64)));
    assert_eq!(parse_go_type("complex64"), Ok(("", GoType::Complex64)));
    assert_eq!(parse_go_type("complex128"), Ok(("", GoType::Complex128)));
    assert_eq!(parse_go_type("byte"), Ok(("", GoType::Byte)));
    assert_eq!(parse_go_type("rune"), Ok(("", GoType::Rune)));
    assert_eq!(parse_go_type("string"), Ok(("", GoType::String)));
}

#[test]
fn test_parameters() {
    let map = hashmap! {"x" => GoType::Int};
    assert_eq!(parse_parameters("(x int)"), Ok(("", ArgTypes(map))));
}
