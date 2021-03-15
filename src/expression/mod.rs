use nom::{branch::alt, IResult};

use crate::parse_util::symbol;

pub enum Expression {
    // UnaryExpr(UnaryExpr),
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
