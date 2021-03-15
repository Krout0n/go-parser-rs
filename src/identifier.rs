use nom::IResult;

use crate::parse_util::{identifier, symbol};

#[derive(Debug, PartialEq)]
pub struct QualifiedIdent<'a>(&'a str, &'a str);
pub fn qualified_ident<'a>(s: &'a str) -> IResult<&'a str, QualifiedIdent<'a>> {
    let (s, pkg_name) = identifier(s)?;
    let (s, _) = symbol('.')(s)?;
    let (s, ident) = identifier(s)?;
    Ok((s, QualifiedIdent(pkg_name, ident)))
}

#[test]
fn test_qualified_ident() {
    assert_eq!(qualified_ident("x.y"), Ok(("", QualifiedIdent("x", "y"))))
}
