use nom::{branch::alt, combinator::map, IResult};

use crate::{
    astable::ASTable, identifier::QualifiedIdent, literals::Literal, parse_util::identifier,
};

use super::Expression;

/// Operand = Literal | OperandName | "(" Expression ")"
pub enum Operand<'a> {
    Literal(Literal<'a>),
    OperandName(OperandName<'a>),
    Parenthesized(Expression),
}

#[derive(Debug, PartialEq)]
pub enum OperandName<'a> {
    Identifier(&'a str),
    QualifiedIdent(QualifiedIdent<'a>),
}

impl<'a> ASTable<'a> for OperandName<'a> {
    ///```
    /// use go_parser_rs::expression::operand::OperandName;
    /// use go_parser_rs::identifier::QualifiedIdent;
    /// use go_parser_rs::astable::ASTable;
    /// assert_eq!(OperandName::parse("x.y"), Ok(("", OperandName::QualifiedIdent(QualifiedIdent{package_name: "x", identifier: "y"}))));
    /// assert_eq!(OperandName::parse("vmw"), Ok(("", OperandName::Identifier("vmw"))));
    ///```
    fn parse(s: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(QualifiedIdent::parse, Self::QualifiedIdent),
            map(identifier, Self::Identifier),
        ))(s)
    }
}
