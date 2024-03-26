use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

/// A parser that automatically backtracks on fail through [`Recoverable`]
///
/// This `struct` is created by the [`Parser::auto_bt`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct AutoBt<Par> {
    parser: Par,
}

impl<Par> AutoBt<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Output: Recoverable,
    {
        Self { parser }
    }
}

impl<Par> Parser for AutoBt<Par>
where
    Par: Parser,
    Par::Output: Recoverable,
{
    type Input = Par::Input;
    type Output = Par::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        let offset = input.offset();
        self.parser.parse_stream(input).recover_response(
            |input| {
                *input.offset_mut() = offset;
            },
            input,
        )
    }
}
