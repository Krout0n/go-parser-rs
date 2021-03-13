use nom::{character::complete::one_of, IResult};

///
/// ```
/// use go_parser_rs::letter_and_digit::parse_decimal_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(parse_decimal_digit("21c"), Ok(("1c", '2')));
/// assert_eq!(parse_decimal_digit("c1"), Err(Err::Error(Error::new("c1", ErrorKind::OneOf))));
/// assert_eq!(parse_decimal_digit(""), Err(Err::Error(Error::new("", ErrorKind::OneOf))));
/// ```
pub fn parse_decimal_digit(s: &str) -> IResult<&str, char> {
    one_of("0123456789")(s)
}

///
/// ```
/// use go_parser_rs::letter_and_digit::parse_octal_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(parse_octal_digit("21c"), Ok(("1c", '2')));
/// assert_eq!(parse_octal_digit("c1"), Err(Err::Error(Error::new("c1", ErrorKind::OneOf))));
/// assert_eq!(parse_octal_digit(""), Err(Err::Error(Error::new("", ErrorKind::OneOf))));
/// ```
pub fn parse_octal_digit(s: &str) -> IResult<&str, char> {
    one_of("01234567")(s)
}

///
/// ```
/// use go_parser_rs::letter_and_digit::parse_binary_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(parse_binary_digit("01c"), Ok(("1c", '0')));
/// assert_eq!(parse_binary_digit("1c"), Ok(("c", '1')));
/// assert_eq!(parse_binary_digit(""), Err(Err::Error(Error::new("", ErrorKind::OneOf))));
/// ```
pub fn parse_binary_digit(s: &str) -> IResult<&str, char> {
    one_of("01")(s)
}

///
/// ```
/// use go_parser_rs::letter_and_digit::parse_hex_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(parse_hex_digit("21c"), Ok(("1c", '2')));
/// assert_eq!(parse_hex_digit("c1"),Ok(("1", 'c')));
/// assert_eq!(parse_hex_digit(""), Err(Err::Error(Error::new("", ErrorKind::OneOf))));
/// ```
pub fn parse_hex_digit(s: &str) -> IResult<&str, char> {
    one_of("0123456789abcdefABCDEF")(s)
}
