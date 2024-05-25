use crate::parser::prelude::*;
use crate::output::prelude::*;
use crate::input::prelude::*;

/// A parser for transforming [`Fallible`]s into Infallible responses
///
/// This `struct` is created by the [`Parser::opt`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct Opt<Par> {
    parser: Par,
}

impl<Par> Opt<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Output: Fallible,
    {
        Self { parser }
    }
}

impl<Par> Parser for Opt<Par>
where
    Par: Parser,
    Par::Output: Fallible,
{
    type Input = Par::Input;
    type Output = <Par::Output as Fallible>::Optional;

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .as_ref()
            .auto_bt()
            .next(input)
            .optional()
    }
}
