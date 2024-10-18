use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

// TODO: Documentation
pub type FnMap<Par, Val0, Val1> = Map<Par, fn(Val0) -> Val1>;

/// A parser for mapping the [`Response::Value`] through [`Mappable`]
///
/// This `struct` is created by the [`Parser::map`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Map<Par, Fun> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) function: Fun,
}

#[parser_fn]
fn map<par, Fun>(self: &Map<par, Fun>) -> <par::Output as Select<Fun>>::Output
where
    par::Output: Select<Fun>,
{
    parse![self.parser].sel(&self.function)
}
