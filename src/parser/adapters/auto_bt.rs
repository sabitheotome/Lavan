use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

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
        Par::Output: Fallible,
    {
        Self { parser }
    }
}

impl<Par> Parser for AutoBt<Par>
where
    Par: Parser,
    Par::Output: Fallible,
{
    type Input = Par::Input;
    type Output = Par::Output;

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        let mut rewind_state = input.savestate();
        self.parser
            .next(input)
            .on_err(|| input.backtrack(rewind_state))
    }
}
