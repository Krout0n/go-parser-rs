use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, space0},
    combinator::not,
    IResult,
};

pub fn reserved(keyword: &'static str) -> impl Fn(&str) -> IResult<&str, ()> {
    move |s: &str| {
        let (s, _) = tag(keyword)(s)?;
        let (s, _) = not(alphanumeric1)(s)?;
        let (s, _) = space0(s)?;
        Ok((s, ()))
    }
}

pub fn symbol(sym: char) -> impl Fn(&str) -> IResult<&str, ()> {
    move |s: &str| {
        let (s, _) = char(sym)(s)?;
        let (s, _) = space0(s)?;
        Ok((s, ()))
    }
}

pub fn identifier(s: &str) -> IResult<&str, &str> {
    let (s, ident) = alphanumeric1(s)?;
    let (s, _) = space0(s)?;
    Ok((s, ident))
}
