use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser for ignoring the [`Response::Error`] through [`ErrIgnorable`]
///
/// This `struct` is created by the [`Parser::ok`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Ok<Par> {
    parser: Par,
}

impl<Par> Ok<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Operator,
        Par::Response: ErrIgnorable,
    {
        Self { parser }
    }
}

impl<Par> Operator for Ok<Par>
where
    Par: Operator,
    Par::Response: ErrIgnorable,
{
    type Scanner = Par::Scanner;
    type Response = <Par::Response as ErrIgnorable>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser.parse_next(input).ignore_err_response()
    }
}
