use nom::{branch::alt, IResult};

use crate::parse_util::symbol;

pub enum Expression {
    // UnaryExpr(UnaryExpr),
}

// Precedence    Operator
//     5             *  /  %  <<  >>  &  &^
//     4             +  -  |  ^
//     3             ==  !=  <  <=  >  >=
//     2             &&
//     1             ||

/// ```
/// use go_parser_rs::expression::and_op;
/// assert_eq!(and_op("&&true"), Ok(("true", "&&")));
/// ```
pub fn and_op(s: &str) -> IResult<&str, &str> {
    symbol("&&")(s)
}

/// rel_op = "==" | "!=" | "<" | "<="| ">" | ">=" .
/// ```
/// use go_parser_rs::expression::rel_op;
/// assert_eq!(rel_op("==1"), Ok(("1", "==")));
/// assert_eq!(rel_op("!=1"), Ok(("1", "!=")));
/// assert_eq!(rel_op("<3"), Ok(("3", "<")));
/// assert_eq!(rel_op("<=a"), Ok(("a", "<=")));
/// assert_eq!(rel_op(">a"), Ok(("a", ">")));
/// assert_eq!(rel_op(">= a"), Ok(("a", ">=")));
/// ```
pub fn rel_op(s: &str) -> IResult<&str, &str> {
    alt((
        symbol("=="),
        symbol("!="),
        symbol("<="),
        symbol(">="),
        symbol("<"),
        symbol(">"),
    ))(s)
}

/// add_op = "+" | "-" | "|" | "^" .
/// ```
/// use go_parser_rs::expression::add_op;
/// assert_eq!(add_op("+1"), Ok(("1", "+")));
/// assert_eq!(add_op("-1"), Ok(("1", "-")));
/// assert_eq!(add_op("|3"), Ok(("3", "|")));
/// assert_eq!(add_op("^a"), Ok(("a", "^")));
/// assert!(add_op("*a").is_err());
/// ```
pub fn add_op(s: &str) -> IResult<&str, &str> {
    alt((symbol("+"), symbol("-"), symbol("|"), symbol("^")))(s)
}

/// mul_op = "*" | "/" | "%" | "<<" | ">>" | "&" | "&^" .
/// ```
/// use go_parser_rs::expression::mul_op;
/// assert_eq!(mul_op("*1"), Ok(("1", "*")));
/// assert_eq!(mul_op("/1"), Ok(("1", "/")));
/// assert_eq!(mul_op("% 2"), Ok(("2", "%")));
/// assert_eq!(mul_op("<<2"), Ok(("2", "<<")));
/// assert_eq!(mul_op(">>1"), Ok(("1", ">>")));
/// assert_eq!(mul_op("& x"), Ok(("x", "&")));
/// assert_eq!(mul_op("&^ x"), Ok(("x", "&^")));
/// ```
pub fn mul_op(s: &str) -> IResult<&str, &str> {
    alt((
        symbol("*"),
        symbol("/"),
        symbol("%"),
        symbol("<<"),
        symbol(">>"),
        symbol("&^"),
        symbol("&"),
    ))(s)
}

/// unary_op   = "+" | "-" | "!" | "^" | "*" | "&" | "<-" .
/// ```
/// use go_parser_rs::expression::unary_op;
/// assert_eq!(unary_op("+1"), Ok(("1", "+")));
/// assert_eq!(unary_op("-1"), Ok(("1", "-")));
/// assert_eq!(unary_op("!true"), Ok(("true", "!")));
/// assert_eq!(unary_op("^a"), Ok(("a", "^")));
/// assert_eq!(unary_op("&1"), Ok(("1", "&")));
/// assert_eq!(unary_op("<- ch"), Ok(("ch", "<-")));
/// ```
pub fn unary_op(s: &str) -> IResult<&str, &str> {
    alt((
        symbol("+"),
        symbol("-"),
        symbol("!"),
        symbol("^"),
        symbol("*"),
        symbol("&"),
        symbol("<-"),
    ))(s)
}
