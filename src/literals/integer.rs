use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{not, opt, recognize},
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

use super::letter_and_digit::{binary_digit, decimal_digit, hex_digit, octal_digit};

/// decimal_digits = decimal_digit { [ "_" ] decimal_digit } .
/// ```
/// use go_parser_rs::literals::integer::decimal_digits;
/// assert_eq!(decimal_digits("17014114105727"), Ok(("", "17014114105727")));
/// assert_eq!(decimal_digits("170_1411_105727"), Ok(("", "170_1411_105727")));
/// assert!(decimal_digits("_42").is_err()); // an identifier, not an integer literal
/// assert!(decimal_digits("42_").is_err()); // invalid: _ must separate successive digits
/// assert!(decimal_digits("4__2").is_err()); // invalid: only one _ at a time
/// ```
pub fn decimal_digits(s: &str) -> IResult<&str, &str> {
    let (s, digits) = recognize(pair(
        decimal_digit,
        many0(pair(opt(tag("_")), decimal_digit)),
    ))(s)?;
    let (s, _) = not(tag("_"))(s)?;
    Ok((s, digits))
}

/// Note: Failed on some test cases now. use this function carefully.
/// decimal_lit    = "0" | ( "1" … "9" ) [ [ "_" ] decimal_digits ].
/// ```
/// use go_parser_rs::literals::integer::decimal_lit;
/// assert_eq!(decimal_lit("0"), Ok(("", "0")));
/// assert_eq!(decimal_lit("123456789"), Ok(("", "123456789")));
/// assert_eq!(decimal_lit("12_3_45_6789"), Ok(("", "12_3_45_6789")));
/// assert!(decimal_lit("0123").is_err()); // TODO: Fix this case, invalid: non zero literal can start with 0.
/// assert!(decimal_lit("00").is_err()); // TODO: Fix this case, invalid: zero literal can't start with multi times 0.
/// assert!(decimal_lit("12__3_45_6789").is_err()); // invalid: only one _ at a time
/// ```
pub fn decimal_lit(s: &str) -> IResult<&str, &str> {
    // ( "1" … "9" )
    let _1to9 = one_of("123456789");
    // [ [ "_" ] decimal_digits ]
    let parser = many0(pair(opt(tag("_")), decimal_digits));
    let (s, digits) = recognize(alt((tag("0"), recognize(pair(_1to9, parser)))))(s)?;
    let (s, _) = not(tag("_"))(s)?;
    Ok((s, digits))
}

/// binary_digits = binary_digit { [ "_" ] binary_digit } .
/// ```
/// use go_parser_rs::literals::integer::binary_digits;
/// assert_eq!(binary_digits("0101011"), Ok(("", "0101011")));
/// assert_eq!(binary_digits("1_01_0_1_010_100111_01_0100010"), Ok(("", "1_01_0_1_010_100111_01_0100010")));
/// assert!(binary_digits("_01").is_err()); // an identifier, not an integer literal
/// assert!(binary_digits("0_1_").is_err()); // invalid: _ must separate successive digits
/// assert!(binary_digits("0__1").is_err()); // invalid: only one _ at a time
/// ```
pub fn binary_digits(s: &str) -> IResult<&str, &str> {
    let (s, digits) = recognize(pair(binary_digit, many0(pair(opt(tag("_")), binary_digit))))(s)?;
    let (s, _) = not(tag("_"))(s)?;
    Ok((s, digits))
}

/// binary_lit = "0" ( "b" | "B" ) [ "_" ] binary_digits .
///
///```
/// use go_parser_rs::literals::integer::binary_lit;
/// assert_eq!(binary_lit("0b010"), Ok(("", "0b010")));
/// assert_eq!(binary_lit("0B101"), Ok(("", "0B101")));
/// assert_eq!(binary_lit("0b_01_1_1_10_0"), Ok(("", "0b_01_1_1_10_0")));
/// assert!(binary_lit("0_B101").is_err()); // invalid: _ must separate successive digits
///```
pub fn binary_lit(s: &str) -> IResult<&str, &str> {
    recognize(tuple((
        tag("0"),
        opt(alt((tag("b"), tag("B")))),
        opt(tag("_")),
        binary_digits,
    )))(s)
}

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
/// assert_eq!(hex_lit("0XBad_Face"), Ok(("", "0XBad_Face")));
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
