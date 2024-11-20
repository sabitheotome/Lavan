use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// A parser for transforming [`Fallible`]s into Infallible responses
///
/// This `struct` is created by the [`Parser::opt`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Lift<Par> {
    pub(in crate::parser) parser: Par,
}

#[parser_fn]
fn lift<par>(self: &Lift<par>) -> Sure<par::Output> {
    Sure(parse![parser![self.parser]])
}