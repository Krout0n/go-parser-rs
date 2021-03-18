pub mod operand;
use nom::{branch::alt, combinator::map, IResult};

use crate::{astable::ASTable, parse_util::symbol};

use self::operand::Operand;
#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    UnaryExpr(UnaryExpr<'a>),
    BinExpr {
        left: Box<Self>,
        op: &'a str,
        right: Box<Self>,
    },
}

// Precedence    Operator
//     5             *  /  %  <<  >>  &  &^
//     4             +  -  |  ^
//     3             ==  !=  <  <=  >  >=
//     2             &&
//     1             ||

#[derive(Debug, PartialEq)]
pub struct Unary<'a> {
    pub op: &'a str,
    pub expr: Box<UnaryExpr<'a>>,
}

impl<'a> ASTable<'a> for Unary<'a> {
    /// ```
    /// use go_parser_rs::astable::ASTable;
    /// use go_parser_rs::expression::{Unary, UnaryExpr, PrimaryExpr, operand::{Operand, OperandName}};
    /// use go_parser_rs::literals::{integer::IntLit, Literal};
    /// assert_eq!(
    ///    Unary::parse("- 1 +2"),
    ///    Ok((
    ///        "+2",
    ///        Unary {
    ///            op: "-",
    ///            expr: Box::new(UnaryExpr::PrimaryExpr(PrimaryExpr::Operand(
    ///                Operand::Literal(Literal::IntLit(IntLit::DecimalLit("1")))
    ///            )))
    ///        }
    ///    ))
    /// );
    /// ```
    fn parse(s: &'a str) -> IResult<&'a str, Self> {
        let (s, op) = unary_op(s)?;
        let (s, expr) = UnaryExpr::parse(s)?;
        Ok((
            s,
            Unary {
                op,
                expr: Box::new(expr),
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryExpr<'a> {
    PrimaryExpr(PrimaryExpr<'a>),
    Unary(Unary<'a>),
}

impl<'a> ASTable<'a> for UnaryExpr<'a> {
    /// ```
    /// use go_parser_rs::astable::ASTable;
    /// use go_parser_rs::expression::{UnaryExpr, PrimaryExpr, operand::{Operand, OperandName}};
    /// use go_parser_rs::literals::{integer::IntLit, Literal};
    /// assert_eq!(UnaryExpr::parse("1+2"), Ok(("+2", UnaryExpr::PrimaryExpr(PrimaryExpr::Operand(Operand::Literal(Literal::IntLit(IntLit::DecimalLit("1"))))))));
    /// ```
    fn parse(s: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(Unary::parse, UnaryExpr::Unary),
            map(PrimaryExpr::parse, UnaryExpr::PrimaryExpr),
        ))(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum PrimaryExpr<'a> {
    Operand(Operand<'a>),
}

impl<'a> ASTable<'a> for PrimaryExpr<'a> {
    /// ```
    /// use go_parser_rs::astable::ASTable;
    /// use go_parser_rs::expression::{PrimaryExpr, operand::{Operand, OperandName}};
    /// use go_parser_rs::literals::{integer::IntLit, Literal};
    /// assert_eq!(PrimaryExpr::parse("1+2"), Ok(("+2", PrimaryExpr::Operand(Operand::Literal(Literal::IntLit(IntLit::DecimalLit("1")))))));
    /// ```
    fn parse(s: &'a str) -> IResult<&'a str, Self> {
        map(Operand::parse, PrimaryExpr::Operand)(s)
    }
}

/// ```
/// use go_parser_rs::expression::or_op;
/// assert_eq!(or_op("||true"), Ok(("true", "||")));
/// ```
pub fn or_op(s: &str) -> IResult<&str, &str> {
    symbol("||")(s)
}

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
