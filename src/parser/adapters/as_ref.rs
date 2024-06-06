use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser for taking another parser by reference
///
/// This `struct` is created by the [`Parser::and`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct AsRef<'a, Par> {
    parser: &'a Par,
}

impl<'a, Par> AsRef<'a, Par> {
    pub(crate) fn new(parser: &'a Par) -> Self
    where
        Par: Operator,
    {
        Self { parser }
    }
}

impl<'a, Par> Operator for AsRef<'a, Par>
where
    Par: Operator,
{
    type Scanner = Par::Scanner;
    type Response = Par::Response;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser.parse_next(input)
    }
}
