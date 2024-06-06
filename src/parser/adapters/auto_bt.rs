use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser that automatically backtracks on fail through [`Recoverable`]
///
/// This `struct` is created by the [`Parser::auto_bt`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct AutoBt<Par> {
    parser: Par,
}

impl<Par> AutoBt<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Operator,
        Par::Response: Fallible,
    {
        Self { parser }
    }
}

impl<Par> Operator for AutoBt<Par>
where
    Par: Operator,
    Par::Response: Fallible,
{
    type Scanner = Par::Scanner;
    type Response = Par::Response;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut rewind_state = input.savestate();
        self.parser
            .parse_next(input)
            .on_err(|| input.backtrack(rewind_state))
    }
}
