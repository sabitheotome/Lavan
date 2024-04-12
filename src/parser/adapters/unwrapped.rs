use std::fmt::Debug;

use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

/// Unwrap the inner value contained in the response, converting it to a infallible response
///
/// This `struct` is created by the [`Parser::unwrapped`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct Unwrapped<Par> {
    parser: Par,
}

impl<Par> Unwrapped<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Output: Fallible + ValueFunctor,
        <Par::Output as Response>::Error: std::fmt::Debug,
    {
        Self { parser }
    }
}

impl<Par> Parser for Unwrapped<Par>
where
    Par: Parser,
    Par::Output: Fallible + ValueFunctor,
    <Par::Output as Response>::Error: std::fmt::Debug,
{
    type Input = Par::Input;
    type Output = <Par::Output as Fallible>::Infallible;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        <Par::Output as Fallible>::Infallible::from_value(self.parser.parse_stream(input).unwrap())
    }
}
