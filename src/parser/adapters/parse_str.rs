use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;
use std::marker::PhantomData;

/// A parser for converting a `str` to `T` where `T: std::str::FromStr`
///
/// This `struct` is created by the [`Parser::parse_str`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct ParseStr<Par, T> {
    parser: Par,
    convert_to: PhantomData<T>,
}

impl<Par, T> ParseStr<Par, T> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Output: Bindable<fn(&str) -> Result<T, T::Err>>,
        T: std::str::FromStr,
    {
        Self {
            parser,
            convert_to: PhantomData,
        }
    }
}

impl<'a, Par, T> Parser for ParseStr<Par, T>
where
    Par: Parser,
    Par::Output: Bindable<fn(&str) -> Result<T, T::Err>>,
    T: std::str::FromStr,
{
    type Input = Par::Input;
    type Output = <Par::Output as Bindable<fn(&str) -> Result<T, T::Err>>>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .as_ref()
            .then(|str: &str| str.parse::<T>())
            .parse_stream(input)
    }
}
