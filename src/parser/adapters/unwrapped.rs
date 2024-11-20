use std::fmt::Debug;

use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// Unwrap the inner value contained in the response, converting it to a infallible response
///
/// This `struct` is created by the [`Parser::unwrapped`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Unwrapped<Par> {
    pub(in crate::parser) parser: Par,
}

#[parser_fn]
fn unwrapped<par>(self: &Unwrapped<par>) -> <par::Output as Fallible>::Infallible
where
    par::Output: Fallible + ValueResponse,
    <par::Output as Response>::Error: std::fmt::Debug,
{
    <par::Output as Fallible>::Infallible::from_value(parse![self.parser].unwrap())
}
