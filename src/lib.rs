pub mod astable;
pub mod expression;
pub mod identifier;
pub mod literals;
mod parse_util;
pub mod tokenize;
pub mod typ;

use nom::{
    bytes::streaming::take_while,
    character::complete::space0,
    combinator::opt,
    multi::many0,
    sequence::tuple,
    sequence::{delimited, preceded},
    IResult,
};

use parse_util::{identifier, reserved, symbol};

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
pub struct Function<'a> {
    pub name: &'a str,
    pub params: Parameters<'a>,
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
    let (s, _) = reserved("package")(s)?;
    let (s, pkg_name) = identifier(s)?;
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
    let (s, _) = space0(s)?;
    Ok((s, GoType::from(typ)))
}

// Thanks to drumato!
// https://github.com/Drumato/peachili/blob/codegen/src/compiler/common/frontend/pass/parser/primitive.rs#L14
fn parse_string_literal(i: &str) -> nom::IResult<&str, &str> {
    let (rest, contents) =
        delimited(symbol("\""), take_while(|b: char| b != '"'), symbol("\""))(i)?;
    Ok((rest, contents))
}

// FunctionDecl = "func" FunctionName Signature [ FunctionBody ] .
// FunctionBody = . // TODO: Implement block.
fn parse_function_decl<'a>(s: &'a str) -> IResult<&'a str, Function<'a>> {
    // func f (x int) string
    let (s, _) = reserved("func")(s)?;
    let (s, name) = identifier(s)?;
    let (s, params) = parse_parameters(s)?;
    let (s, ret) = parse_go_type(s)?;
    let (s, _) = space0(s)?;
    Ok((s, Function { name, params, ret }))
}

#[derive(Debug, PartialEq)]
pub struct Parameters<'a>(Option<ParameterList<'a>>);
// Parameters     = "(" [ ParameterList [ "," ] ] ")" .
fn parse_parameters(s: &str) -> IResult<&str, Parameters> {
    let parse_parameter_list_opt = opt(tuple((parse_parameter_list, opt(symbol(",")))));
    let (s, parameter_list_opt) = delimited(symbol("("), parse_parameter_list_opt, symbol(")"))(s)?;
    let parameter_list = parameter_list_opt.map(|(s, _)| s);
    Ok((s, Parameters(parameter_list)))
}

#[derive(Debug, PartialEq)]
pub struct ParameterList<'a>(Vec<ParameterDecl<'a>>);
// ParameterList  = ParameterDecl { "," ParameterDecl } .
fn parse_parameter_list<'a>(s: &'a str) -> IResult<&'a str, ParameterList<'a>> {
    let (s, f) = parse_parameter_decl(s)?;
    let (s, mut decls) = many0(preceded(symbol(","), parse_parameter_decl))(s)?;
    decls.insert(0, f);
    Ok((s, ParameterList(decls)))
}

#[derive(Debug, PartialEq)]
pub struct ParameterDecl<'a> {
    identifiers: Option<Vec<&'a str>>,
    is_variadic: bool,
    go_type: GoType,
}
// ParameterDecl  = [ IdentifierList ] [ "..." ] Type .
fn parse_parameter_decl(s: &str) -> IResult<&str, ParameterDecl> {
    let (s, identifiers) = opt(parse_identifier_list)(s)?;
    let (s, is_variadic_opt) = opt(reserved("..."))(s)?;
    let is_variadic = is_variadic_opt.is_some();
    let (s, go_type) = parse_go_type(s)?;
    Ok((
        s,
        ParameterDecl {
            identifiers,
            is_variadic,
            go_type,
        },
    ))
}

// IdentifierList = identifier { "," identifier } .
fn parse_identifier_list(s: &str) -> IResult<&str, Vec<&str>> {
    let (s, i) = identifier(s)?;
    let (s, mut result) = many0(preceded(symbol(","), identifier))(s)?;
    result.insert(0, i);
    Ok((s, result))
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
fn test_func_decl() {
    assert_eq!(
        parse_function_decl("func f (x int) string"),
        Ok((
            "",
            Function {
                name: "f",
                params: Parameters(Some(ParameterList(vec![ParameterDecl {
                    identifiers: Some(vec!["x"]),
                    is_variadic: false,
                    go_type: GoType::Int
                }]))),
                ret: GoType::String
            }
        ))
    );
}

#[test]
fn test_parameters() {
    assert_eq!(
        parse_parameters("(x int)"),
        Ok((
            "",
            Parameters(Some(ParameterList(vec![ParameterDecl {
                identifiers: Some(vec!["x"]),
                is_variadic: false,
                go_type: GoType::Int
            }])))
        ))
    );

    assert_eq!(
        parse_parameters("(x, y int, z string)"),
        Ok((
            "",
            Parameters(Some(ParameterList(vec![
                ParameterDecl {
                    identifiers: Some(vec!["x", "y"]),
                    is_variadic: false,
                    go_type: GoType::Int
                },
                ParameterDecl {
                    identifiers: Some(vec!["z"]),
                    is_variadic: false,
                    go_type: GoType::String
                }
            ])))
        ))
    );
}

#[test]
fn test_parameter_list() {
    assert_eq!(
        parse_parameter_list("x int"),
        Ok((
            "",
            ParameterList(vec![ParameterDecl {
                identifiers: Some(vec!["x"]),
                is_variadic: false,
                go_type: GoType::Int
            }])
        ))
    );

    assert_eq!(
        parse_parameter_list("x, y int, z string"),
        Ok((
            "",
            ParameterList(vec![
                ParameterDecl {
                    identifiers: Some(vec!["x", "y"]),
                    is_variadic: false,
                    go_type: GoType::Int
                },
                ParameterDecl {
                    identifiers: Some(vec!["z"]),
                    is_variadic: false,
                    go_type: GoType::String
                }
            ])
        ))
    );
}

#[test]
fn test_parameter_decl() {
    assert_eq!(
        parse_parameter_decl("x int"),
        Ok((
            "",
            ParameterDecl {
                identifiers: Some(vec!["x"]),
                is_variadic: false,
                go_type: GoType::Int
            }
        ))
    );

    assert_eq!(
        parse_parameter_decl("x, y int"),
        Ok((
            "",
            ParameterDecl {
                identifiers: Some(vec!["x", "y"]),
                is_variadic: false,
                go_type: GoType::Int
            }
        ))
    );

    assert_eq!(
        parse_parameter_decl("x, y ... int"),
        Ok((
            "",
            ParameterDecl {
                identifiers: Some(vec!["x", "y"]),
                is_variadic: true,
                go_type: GoType::Int
            }
        ))
    );
}

#[test]
fn test_identifier_list() {
    assert_eq!(parse_identifier_list("x"), Ok(("", vec!["x"])));
    assert_eq!(parse_identifier_list("x, y"), Ok(("", vec!["x", "y"])));
    assert_eq!(parse_identifier_list("x, y z"), Ok(("z", vec!["x", "y"])));
}
