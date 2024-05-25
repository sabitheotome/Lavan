use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

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

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.next(input)
    }
}

impl<'a, Par> IntoParser<<AsRef<'a, Par> as Parser>::Input, <AsRef<'a, Par> as Parser>::Output>
    for AsRef<'a, Par>
where
    Par: Parser,
{
    type IntoParser = Self;

    fn into_parser(self) -> Self::IntoParser {
        self
    }
}
