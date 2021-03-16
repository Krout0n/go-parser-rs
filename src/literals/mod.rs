use nom::{combinator::map, IResult};

use crate::astable::ASTable;

use self::integer::{int_lit, IntLit};

pub mod integer;
pub mod letter_and_digit;

///
/// Literal     = BasicLit | CompositeLit | FunctionLit .
/// OperandName = identifier | QualifiedIdent .
pub enum Literal<'a> {
    /// BasicLit    = int_lit | float_lit | imaginary_lit | rune_lit | string_lit .
    IntLit(IntLit<'a>),
    FloatLit,
    ImaginaryLit,
    RuneLit,
    StringLit(&'a str),
    // TODO: Composite, FunctionLit...
}

impl<'a> Literal<'a> {
    fn parse_int_lit(s: &'a str) -> IResult<&'a str, Self> {
        map(int_lit, Self::IntLit)(s)
    }
}

impl<'a> ASTable<'a> for Literal<'a> {
    fn parse(s: &'a str) -> IResult<&'a str, Self> {
        Self::parse_int_lit(s)
    }
}
