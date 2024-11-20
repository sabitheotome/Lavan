use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;
use std::marker::PhantomData;

/// A parser for flat-mapping responses
///
/// This `struct` is created by the [`Parser::then`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Then<Par, Fun> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) function: Fun,
}

#[parser_fn]
fn then<par, Fun>(self: &Then<par, Fun>) -> <par::Output as Apply<Fun>>::Output
where
    par::Output: Apply<Fun>,
{
    parse![self.parser].apply(&self.function)
}
