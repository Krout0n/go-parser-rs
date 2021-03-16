use crate::{identifier::QualifiedIdent, literals::Literal};

use super::Expression;

/// Operand = Literal | OperandName | "(" Expression ")"
pub enum Operand<'a> {
    Literal(Literal<'a>),
    OperandName(OperandName<'a>),
    Parenthesized(Expression),
}

pub enum OperandName<'a> {
    Identifier(&'a str),
    QualifiedIdent(QualifiedIdent<'a>),
}
