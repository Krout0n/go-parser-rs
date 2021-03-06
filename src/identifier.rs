use nom::IResult;

use crate::{
    astable::ASTable,
    parse_util::{identifier, symbol},
};

#[derive(Debug, PartialEq)]
pub struct QualifiedIdent<'a> {
    pub package_name: &'a str,
    pub identifier: &'a str,
}

impl<'a> ASTable<'a> for QualifiedIdent<'a> {
    /// QualifiedIdent = PackageName "." identifier .
    /// ```
    /// use go_parser_rs::astable::ASTable;
    /// use go_parser_rs::identifier::QualifiedIdent;
    /// assert_eq!(QualifiedIdent::parse("x.y"), Ok(("", QualifiedIdent{ package_name: "x", identifier: "y" })));
    /// ```
    fn parse(s: &'a str) -> IResult<&'a str, QualifiedIdent<'a>> {
        let (s, package_name) = identifier(s)?;
        let (s, _) = symbol(".")(s)?;
        let (s, identifier) = identifier(s)?;
        Ok((
            s,
            QualifiedIdent {
                package_name,
                identifier,
            },
        ))
    }
}
