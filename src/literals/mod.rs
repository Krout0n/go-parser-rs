use nom::{branch::alt, combinator::map, IResult};

use crate::astable::ASTable;

use self::{
    integer::{int_lit, IntLit},
    rune::Rune,
};

pub mod integer;
pub mod letter_and_digit;
pub mod rune;

///
/// Literal     = BasicLit | CompositeLit | FunctionLit .
/// OperandName = identifier | QualifiedIdent .
pub enum Literal<'a> {
    /// BasicLit    = int_lit | float_lit | imaginary_lit | rune_lit | string_lit .
    IntLit(IntLit<'a>),
    FloatLit,
    ImaginaryLit,
    RuneLit(Rune),
    StringLit(&'a str),
    // TODO: Composite, FunctionLit...
}

impl<'a> Literal<'a> {
    fn parse_int_lit(s: &'a str) -> IResult<&'a str, Self> {
        map(int_lit, Self::IntLit)(s)
    }

    fn parse_rune_lit(s: &'a str) -> IResult<&'a str, Self> {
        map(Rune::parse, Self::RuneLit)(s)
    }
}

impl<'a> ASTable<'a> for Literal<'a> {
    fn parse(s: &'a str) -> IResult<&'a str, Self> {
        alt((Self::parse_int_lit, Self::parse_rune_lit))(s)
    }
}
