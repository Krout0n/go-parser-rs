use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::{character::complete::alpha1, IResult};

#[derive(Debug, PartialEq)]
pub enum TopLevel {
    Pkg(String),
}

// PackageClause  = "package" PackageName .
// PackageName    = identifier .
pub fn parse_package_clause(s: &str) -> IResult<&str, TopLevel> {
    let (s, _) = tag("package")(s)?;
    let (s, _) = space1(s)?;
    let (s, pkg_name) = alpha1(s)?;
    Ok((s, TopLevel::Pkg(pkg_name.into())))
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
