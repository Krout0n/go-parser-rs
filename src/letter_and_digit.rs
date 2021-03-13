use nom::{character::complete::digit1, IResult};

///
/// ```
/// use go_parser_rs::letter_and_digit::parse_decimal_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(parse_decimal_digit("21c"), Ok(("c", "21")));
/// assert_eq!(parse_decimal_digit("c1"), Err(Err::Error(Error::new("c1", ErrorKind::Digit))));
/// assert_eq!(parse_decimal_digit(""), Err(Err::Error(Error::new("", ErrorKind::Digit))));
/// ```
pub fn parse_decimal_digit(s: &str) -> IResult<&str, &str> {
    digit1(s)
}
