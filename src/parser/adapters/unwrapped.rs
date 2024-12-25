use crate::parser::prelude::internal::*;

use std::fmt::Debug;

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
