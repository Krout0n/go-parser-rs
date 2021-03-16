use nom::{
    character::complete::{anychar, char},
    sequence::delimited,
    IResult,
};

use crate::astable::ASTable;

#[derive(Debug, PartialEq)]
/// NOTE: char is used as value, but perhaps it will be &str in future because of escaping char.
pub struct Rune(pub char);

impl<'a> ASTable<'a> for Rune {
    ///
    ///```
    /// use go_parser_rs::literals::rune::Rune;
    /// use go_parser_rs::astable::ASTable;
    /// assert_eq!(Rune::parse("'a'bcd"), Ok(("bcd", Rune('a'))));
    ///```
    fn parse(s: &'a str) -> IResult<&'a str, Self> {
        let (s, c) = delimited(char('\''), anychar, char('\''))(s)?;
        Ok((s, Self(c)))
    }
}
