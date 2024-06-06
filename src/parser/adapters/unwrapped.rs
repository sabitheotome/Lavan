use std::fmt::Debug;

use crate::parser::prelude::*;
use crate::output::prelude::*;
use crate::input::prelude::*;

/// Unwrap the inner value contained in the response, converting it to a infallible response
///
/// This `struct` is created by the [`Parser::unwrapped`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Unwrapped<Par> {
    parser: Par,
}

impl<Par> Unwrapped<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Operator,
        Par::Response: Fallible + ValueFunctor,
        <Par::Response as Response>::Error: std::fmt::Debug,
    {
        Self { parser }
    }
}

impl<Par> Operator for Unwrapped<Par>
where
    Par: Operator,
    Par::Response: Fallible + ValueFunctor,
    <Par::Response as Response>::Error: std::fmt::Debug,
{
    type Scanner = Par::Scanner;
    type Response = <Par::Response as Fallible>::Infallible;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        <Par::Response as Fallible>::Infallible::from_value(self.parser.parse_next(input).unwrap())
    }
}
