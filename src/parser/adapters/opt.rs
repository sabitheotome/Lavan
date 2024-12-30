use crate::parser::prelude::internal::*;

/// A parser for transforming [`Fallible`]s into Infallible responses
///
/// This `struct` is created by the [`Parser::opt`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Opt<Par> {
    pub(in crate::parser) parser: Par,
}

#[parser_fn]
fn opt<par>(self: &Opt<par>) -> <par::Output as Fallible>::Optional
where
    par::Output: Fallible,
{
    parse![parser![self.parser].auto_bt()].optional()
}
