use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// A parser that automatically backtracks on fail through [`Recoverable`]
///
/// This `struct` is created by the [`Parser::auto_bt`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct AutoBt<Par> {
    pub(in crate::parser) parser: Par,
}

#[parser_fn]
fn auto_bt<par>(self: &AutoBt<par>) -> par::Output {
    let mut rewind_state = input.savestate();
    parse![self.parser].on_err(|| input.backtrack(rewind_state))
}
