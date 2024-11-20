use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// A parser for ignoring the [`Response::Value`] through [`Ignorable`]
///
/// This `struct` is created by the [`Parser::discard`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Del<Par> {
    pub(in crate::parser) parser: Par,
}

#[parser_fn]
fn del<par>(self: &Del<par>) -> <par::Output as ValueResponse>::VoidVal
where
    par::Output: ValueResponse,
{
    parse![self.parser].void_val()
}
