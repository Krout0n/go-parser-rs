use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0},
    combinator::not,
    IResult,
};

pub fn reserved(keyword: &'static str) -> impl Fn(&str) -> IResult<&str, &str> {
    move |s: &str| {
        let (s, keyword) = tag(keyword)(s)?;
        let (s, _) = not(alphanumeric1)(s)?;
        let (s, _) = space0(s)?;
        Ok((s, keyword))
    }
}

pub fn symbol(sym: &'static str) -> impl Fn(&str) -> IResult<&str, &str> {
    move |s: &str| {
        let (s, _) = tag(sym)(s)?;
        let (s, _) = space0(s)?;
        Ok((s, sym))
    }
}

pub fn identifier(s: &str) -> IResult<&str, &str> {
    let (s, ident) = alphanumeric1(s)?;
    let (s, _) = space0(s)?;
    Ok((s, ident))
}
