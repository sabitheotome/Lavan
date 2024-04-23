use super::traits::Parser;
use crate::{
    response::prelude::Sure,
    stream::traits::{Stream, StreamSlice},
};
use std::marker::PhantomData;

/// A parser for expecting the next token to be an **End of File**
///
/// This `struct` is created by the [`eoi`] method on [`sources`](crate::parser::sources).
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct EOI<Str>(PhantomData<Str>);

// TODO: Documentation
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

/// A parser for expectingany  kind of any token besides **End of File**
///
/// This `struct` is created by the [`any`] method on [`sources`](crate::parser::sources).
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct Any<Str>(PhantomData<Str>);

// TODO: Documentation
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
    type Output = Option<Str::Token>;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        input.next()
    }
}

/// A parser for expecting any token that matches a predicate
///
/// This `struct` is created by the [`any_if`] method on [`sources`](crate::parser::sources).
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct AnyIf<Str, Fun>(PhantomData<Str>, Fun);

// TODO: Documentation
pub fn any_if<Str, Fun>(f: Fun) -> AnyIf<Str, Fun>
where
    Str: Stream,
    Fun: Fn(&Str::Token) -> bool,
{
    AnyIf(PhantomData, f)
}

impl<Str, Fun> Parser for AnyIf<Str, Fun>
where
    Str: Stream,
    Fun: Fn(&Str::Token) -> bool,
{
    type Input = Str;
    type Output = Option<Str::Token>;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        any().filter(&self.1).parse_stream(input)
    }
}

/// A parser for expecting a token to be equal to the provided value
///
/// This `struct` is created by the [`any_eq`] method on [`sources`](crate::parser::sources).
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct AnyEq<Str, const I: bool = false>(PhantomData<Str>, Str::Token)
where
    Str: Stream;

// TODO: Documentation
pub fn any_eq<Str>(v: Str::Token) -> AnyEq<Str>
where
    Str: Stream,
{
    AnyEq(PhantomData, v)
}

impl<Str> Parser for AnyEq<Str>
where
    Str: Stream,
    Str::Token: PartialEq,
{
    type Input = Str;
    type Output = Option<Str::Token>;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        any().filter(|v| *v == self.1).parse_stream(input)
    }
}

/// A parser for expecting a token to be not equal to the provided value
///
/// This `struct` is created by the [`any_ne`] method on [`sources`](crate::parser::sources).
/// See its documentation for more.
type AnyNe<Str> = AnyEq<Str, true>;

// TODO: Documentation
pub fn any_ne<Str>(v: Str::Token) -> AnyNe<Str>
where
    Str: Stream,
{
    AnyEq(PhantomData, v)
}

impl<Str> Parser for AnyNe<Str>
where
    Str: Stream,
    Str::Token: PartialEq,
{
    type Input = Str;
    type Output = Option<Str::Token>;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        any().filter(|v| *v != self.1).parse_stream(input)
    }
}

/// A parser for taking a provided amount of tokens,
/// returning a stream slice starting from the current offset
///
/// This `struct` is created by the [`take`] method on [`sources`](crate::parser::sources).
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct Take<'a, Str>(Str::Offset, PhantomData<&'a Str>)
where
    Str: StreamSlice<'a>;

// TODO: Documentation
pub fn take<'a, Str>(size: Str::Offset) -> Take<'a, Str>
where
    Str: StreamSlice<'a>,
{
    Take(size, PhantomData)
}

impl<'a, Str> Parser for Take<'a, Str>
where
    Str: StreamSlice<'a>,
{
    type Input = Str;
    type Output = Sure<Str::Slice>;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        Sure(input.slice(input.offset(), self.0.clone()))
    }
}
