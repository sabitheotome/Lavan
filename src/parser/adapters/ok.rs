use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// A parser for ignoring the [`Response::Error`] through [`ErrIgnorable`]
///
/// This `struct` is created by the [`Parser::ok`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Ok<Par> {
    pub(in crate::parser) parser: Par,
}

#[parser_fn]
fn ok<par>(self: &Ok<par>) -> <par::Output as ErrorResponse>::VoidErr
where
    par::Output: ErrorResponse,
{
    parse![self.parser].void_err()
}
