use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

/// A parser for taking another parser by reference
///
/// This `struct` is created by the [`Parser::and`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct AsRef<'a, Par> {
    parser: &'a Par,
}

impl<'a, Par> AsRef<'a, Par> {
    pub(crate) fn new(parser: &'a Par) -> Self
    where
        Par: Parser,
    {
        Self { parser }
    }
}

impl<'a, Par> Parser for AsRef<'a, Par>
where
    Par: Parser,
{
    type Input = Par::Input;
    type Output = Par::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.parse_stream(input)
    }
}
