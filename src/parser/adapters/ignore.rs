use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser for ignoring the [`Response::Value`] through [`Ignorable`]
///
/// This `struct` is created by the [`Parser::discard`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct Discard<Par> {
    parser: Par,
}

impl<Par> Discard<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Output: Ignorable,
    {
        Self { parser }
    }
}

impl<Par> Parser for Discard<Par>
where
    Par: Parser,
    Par::Output: Ignorable,
{
    type Input = Par::Input;
    type Output = <Par::Output as Ignorable>::Output;

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.next(input).ignore_response()
    }
}
