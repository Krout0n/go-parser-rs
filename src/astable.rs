use nom::IResult;
pub trait ASTable<'a> {
    fn parse(s: &'a str) -> IResult<&'a str, Self>
    where
        Self: Sized;
}
