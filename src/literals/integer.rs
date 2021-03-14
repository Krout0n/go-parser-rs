use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{not, opt, recognize},
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

use super::letter_and_digit::{hex_digit, octal_digit};

/// octal_digits = octal_digit { [ "_" ] octal_digit }.
/// ```
/// use go_parser_rs::literals::integer::octal_digits;
/// assert_eq!(octal_digits("17014114105727"), Ok(("", "17014114105727")));
/// assert_eq!(octal_digits("170_1411_105727"), Ok(("", "170_1411_105727")));
/// assert!(octal_digits("_42").is_err()); // an identifier, not an integer literal
/// assert!(octal_digits("42_").is_err()); // invalid: _ must separate successive digits
/// assert!(octal_digits("4__2").is_err()); // invalid: only one _ at a time
/// ```
pub fn octal_digits(s: &str) -> IResult<&str, &str> {
    let (s, digits) = recognize(pair(octal_digit, many0(pair(opt(tag("_")), octal_digit))))(s)?;
    let (s, _) = not(tag("_"))(s)?;
    Ok((s, digits))
}

/// octal_lit = "0" [ "o" | "O" ] [ "_" ] octal_digits .
///
///```
/// use go_parser_rs::literals::integer::octal_lit;
/// assert_eq!(octal_lit("012347"), Ok(("", "012347")));
/// assert_eq!(octal_lit("0o600"), Ok(("", "0o600")));
/// assert_eq!(octal_lit("0O600"), Ok(("", "0O600")));
/// assert_eq!(octal_lit("0o_67_7_2_40_6"), Ok(("", "0o_67_7_2_40_6")));
/// assert!(octal_lit("0_O123457").is_err()); // invalid: _ must separate successive digits
///```
pub fn octal_lit(s: &str) -> IResult<&str, &str> {
    recognize(tuple((
        tag("0"),
        opt(alt((tag("o"), tag("O")))),
        opt(tag("_")),
        octal_digits,
    )))(s)
}

/// hex_digits = hex_digit { [ "_" ] hex_digit } .
///
/// ```
/// use go_parser_rs::literals::integer::hex_digits;
/// assert_eq!(hex_digits("170141183460469231731687303715884105727"), Ok(("", "170141183460469231731687303715884105727")));
/// assert_eq!(hex_digits("170_141183_460469_231731_687303_715884_105727"), Ok(("", "170_141183_460469_231731_687303_715884_105727")));
/// assert!(hex_digits("_42").is_err()); // an identifier, not an integer literal
/// assert!(hex_digits("42_").is_err()); // invalid: _ must separate successive digits
/// assert!(hex_digits("4__2").is_err()); // invalid: only one _ at a time
/// ```
pub fn hex_digits(s: &str) -> IResult<&str, &str> {
    let (s, digits) = recognize(pair(hex_digit, many0(pair(opt(tag("_")), hex_digit))))(s)?;
    let (s, _) = not(tag("_"))(s)?;
    Ok((s, digits))
}

/// hex_lit = "0" ( "x" | "X" ) [ "_" ] hex_digits .
///
///```
/// use go_parser_rs::literals::integer::hex_lit;
/// assert_eq!(hex_lit("0xBadFace"), Ok(("", "0xBadFace")));
/// assert_eq!(hex_lit("0xBad_Face"), Ok(("", "0xBad_Face")));
/// assert_eq!(hex_lit("0x_67_7a_2f_cc_40_c6"), Ok(("", "0x_67_7a_2f_cc_40_c6")));
/// assert!(hex_lit("0_xBadFace").is_err()); // invalid: _ must separate successive digits
///```
pub fn hex_lit(s: &str) -> IResult<&str, &str> {
    recognize(tuple((
        tag("0"),
        alt((tag("x"), tag("X"))),
        opt(tag("_")),
        hex_digits,
    )))(s)
}
