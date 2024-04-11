use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

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
        Par::Output: Optionable,
    {
        Self { parser }
    }
}

impl<Par> Parser for Opt<Par>
where
    Par: Parser,
    Par::Output: Optionable,
{
    type Input = Par::Input;
    type Output = <Par::Output as Optionable>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .as_ref()
            .auto_bt()
            .parse_stream(input)
            .opt_response()
    }
}
