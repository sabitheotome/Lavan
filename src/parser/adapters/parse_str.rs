use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;
use std::marker::PhantomData;

/// A parser for converting a `str` to `T` where `T: std::str::FromStr`
///
/// This `struct` is created by the [`Parser::parse_str`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct ParseStr<Par, T> {
    parser: Par,
    convert_to: PhantomData<T>,
}

impl<Par, T> ParseStr<Par, T> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Operator,
        Par::Response: Apply<fn(&str) -> Result<T, T::Err>>,
        T: std::str::FromStr,
    {
        Self {
            parser,
            convert_to: PhantomData,
        }
    }
}

impl<'a, Par, T> Operator for ParseStr<Par, T>
where
    Par: Operator,
    Par::Response: Apply<fn(&str) -> Result<T, T::Err>>,
    T: std::str::FromStr,
{
    type Scanner = Par::Scanner;
    type Response = <Par::Response as Apply<fn(&str) -> Result<T, T::Err>>>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .as_ref()
            .then(|str: &str| str.parse::<T>())
            .parse_next(input)
    }
}
