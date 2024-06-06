use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser for transforming [`Fallible`]s into Infallible responses
///
/// This `struct` is created by the [`Parser::opt`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Opt<Par> {
    parser: Par,
}

impl<Par> Opt<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Operator,
        Par::Response: Fallible,
    {
        Self { parser }
    }
}

impl<Par> Operator for Opt<Par>
where
    Par: Operator,
    Par::Response: Fallible,
{
    type Scanner = Par::Scanner;
    type Response = <Par::Response as Fallible>::Optional;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser.as_ref().auto_bt().parse_next(input).optional()
    }
}
