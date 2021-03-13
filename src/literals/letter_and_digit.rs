use nom::{character::complete::one_of, IResult};

///
/// ```
/// use go_parser_rs::literals::letter_and_digit::decimal_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(decimal_digit("21c"), Ok(("1c", '2')));
/// assert_eq!(decimal_digit("c1"), Err(Err::Error(Error::new("c1", ErrorKind::OneOf))));
/// assert_eq!(decimal_digit(""), Err(Err::Error(Error::new("", ErrorKind::OneOf))));
/// ```
pub fn decimal_digit(s: &str) -> IResult<&str, char> {
    one_of("0123456789")(s)
}

///
/// ```
/// use go_parser_rs::literals::letter_and_digit::octal_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(octal_digit("21c"), Ok(("1c", '2')));
/// assert_eq!(octal_digit("c1"), Err(Err::Error(Error::new("c1", ErrorKind::OneOf))));
/// assert_eq!(octal_digit(""), Err(Err::Error(Error::new("", ErrorKind::OneOf))));
/// ```
pub fn octal_digit(s: &str) -> IResult<&str, char> {
    one_of("01234567")(s)
}

///
/// ```
/// use go_parser_rs::literals::letter_and_digit::binary_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(binary_digit("01c"), Ok(("1c", '0')));
/// assert_eq!(binary_digit("1c"), Ok(("c", '1')));
/// assert_eq!(binary_digit(""), Err(Err::Error(Error::new("", ErrorKind::OneOf))));
/// ```
pub fn binary_digit(s: &str) -> IResult<&str, char> {
    one_of("01")(s)
}

///
/// ```
/// use go_parser_rs::literals::letter_and_digit::hex_digit;
/// use nom::{Err, error::{Error,ErrorKind}};
/// assert_eq!(hex_digit("21c"), Ok(("1c", '2')));
/// assert_eq!(hex_digit("c1"),Ok(("1", 'c')));
/// assert_eq!(hex_digit(""), Err(Err::Error(Error::new("", ErrorKind::OneOf))));
/// ```
pub fn hex_digit(s: &str) -> IResult<&str, char> {
    one_of("0123456789abcdefABCDEF")(s)
}
