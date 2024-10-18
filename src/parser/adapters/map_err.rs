use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

// TODO: Documentation
pub type FnMapErr<Par, Val0, Val1> = MapErr<Par, fn(Val0) -> Val1>;

/// A parser for mapping the [`Response::Error`] through [`ErrMappable`]
///
/// This `struct` is created by the [`Parser::map_err`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct MapErr<Par, Fun> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) function: Fun,
}

#[parser_fn]
fn map_err<par, Fun>(self: &MapErr<par, Fun>) -> <par::Output as SelectErr<Fun>>::Output
where
    par::Output: SelectErr<Fun>,
{
    parse![self.parser].sel_err(&self.function)
}
