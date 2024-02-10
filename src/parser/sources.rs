use super::traits::Parser;
use crate::stream::traits::Stream;
use std::marker::PhantomData;

pub struct Any<Str>(PhantomData<Str>);

pub fn any<Str>() -> Any<Str>
where
    Str: Stream,
{
    Any(PhantomData)
}

impl<Str> Parser for Any<Str>
where
    Str: Stream,
{
    type Input = Str;
    type Output = Option<Str::Item>;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        input.next()
    }
}

pub struct AnyIf<Str, Fun>(PhantomData<Str>, Fun);

pub fn any_if<Str, Fun>(f: Fun) -> AnyIf<Str, Fun>
where
    Str: Stream,
    Fun: Fn(&Str::Item) -> bool,
{
    AnyIf(PhantomData, f)
}

impl<Str, Fun> Parser for AnyIf<Str, Fun>
where
    Str: Stream,
    Fun: Fn(&Str::Item) -> bool,
{
    type Input = Str;
    type Output = Option<Str::Item>;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        any().filter(&self.1).parse_stream(input)
    }
}

pub struct EOI<Str>(PhantomData<Str>);

pub fn eoi<Str>() -> EOI<Str>
where
    Str: Stream,
{
    EOI(PhantomData)
}

impl<Str> Parser for EOI<Str>
where
    Str: Stream,
{
    type Input = Str;
    type Output = bool;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        input.has_next()
    }
}
