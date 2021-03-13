use nom::{
    character::complete::{digit1, one_of},
    combinator::recognize,
    multi::many1,
    IResult,
};

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

///
/// ```
/// use go_parser_rs::letter_and_digit::parse_octal_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(parse_octal_digit("21c"), Ok(("c", "21")));
/// assert_eq!(parse_octal_digit("c1"), Err(Err::Error(Error::new("c1", ErrorKind::OneOf))));
/// assert_eq!(parse_octal_digit(""), Err(Err::Error(Error::new("", ErrorKind::OneOf))));
/// ```
pub fn parse_octal_digit(s: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456")))(s)
}
