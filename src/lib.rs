use nom::bytes::complete::tag;
use nom::{
    bytes::streaming::take_while,
    character::complete::{char, space1},
    sequence::delimited,
};
use nom::{character::complete::alpha1, IResult};

#[derive(Debug, PartialEq)]
pub enum TopLevel {
    Pkg(String),
    Import(Vec<String>),
}

// PackageClause  = "package" PackageName .
// PackageName    = identifier .
pub fn parse_package_clause(s: &str) -> IResult<&str, TopLevel> {
    let (s, _) = tag("package")(s)?;
    let (s, _) = space1(s)?;
    let (s, pkg_name) = alpha1(s)?;
    Ok((s, TopLevel::Pkg(pkg_name.into())))
}

// ImportDecl       = "import" ( ImportSpec | "(" { ImportSpec ";" } ")" ) .
// ImportSpec       = [ "." | PackageName ] ImportPath .
// ImportPath       = string_lit .
pub fn parse_import_decl(s: &str) -> IResult<&str, TopLevel> {
    let (s, _) = tag("import")(s)?;
    let (s, _) = space1(s)?;
    // TODO: multiple '(' packages ')'
    let (s, pkg_path) = parse_string_literal(s)?;
    Ok((s, TopLevel::Import(vec![pkg_path.into()])))
}

// Thanks to drumato!
// https://github.com/Drumato/peachili/blob/codegen/src/compiler/common/frontend/pass/parser/primitive.rs#L14
fn parse_string_literal(i: &str) -> nom::IResult<&str, &str> {
    let (rest, contents) = delimited(char('"'), take_while(|b: char| b != '"'), char('"'))(i)?;
    Ok((rest, contents))
}

#[test]
fn test_pkg_stmt() {
    assert_eq!(
        parse_package_clause("package main"),
        Ok(("", TopLevel::Pkg("main".into())))
    );

    assert_eq!(
        parse_package_clause("package main\n"),
        Ok(("\n", TopLevel::Pkg("main".into())))
    );

    assert!(parse_package_clause("packagemain").is_err())
}

#[test]
fn test_import_decl() {
    // import   "lib/math"
    assert_eq!(
        parse_import_decl("import \"lib/math\""),
        Ok(("", TopLevel::Import(vec!["lib/math".into()])))
    );

    // import m "lib/math"
    // import . "lib/math"
}
