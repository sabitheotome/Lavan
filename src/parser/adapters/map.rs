use crate::parser::prelude::internal::*;

// TODO: Documentation
pub type SelFn<Par, Val0, Val1> = Sel<Par, fn(Val0) -> Val1>;

// TODO: Documentation
pub type SelErrFn<Par, Val0, Val1> = SelErr<Par, fn(Val0) -> Val1>;

/// A parser for mapping the [`Response::Value`] through [`Mappable`]
///
/// This `struct` is created by the [`Parser::map`] method on [`Parser`].
/// See its documentation for more.
#[stability::unstable(feature = "name-tbd")]
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Map<Par, Fun> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) function: Fun,
}

/// A parser for mapping the [`Response::Value`] through [`Mappable`]
///
/// This `struct` is created by the [`Parser::map`] method on [`Parser`].
/// See its documentation for more.
#[stability::unstable(feature = "name-tbd")]
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct MapErr<Par, Fun> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) function: Fun,
}

/// A parser for mapping the [`Response::Value`] through [`Mappable`]
///
/// This `struct` is created by the [`Parser::map`] method on [`Parser`].
/// See its documentation for more.
#[stability::unstable(feature = "name-tbd")]
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Sel<Par, Fun> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) function: Fun,
}

/// A parser for mapping the [`Response::Error`] through [`ErrMappable`]
///
/// This `struct` is created by the [`Parser::map_err`] method on [`Parser`].
/// See its documentation for more.
#[stability::unstable(feature = "name-tbd")]
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct SelErr<Par, Fun> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) function: Fun,
}

#[parser_fn]
fn map<par, Fun, Val>(self: &Map<par, Fun>) -> val![par<Val>]
where
    for<'impl_move> Fun: FnOnce(val![par]) -> Val,
    for<'impl_mut> Fun: FnMut(val![par]) -> Val,
    for<'impl_ref> Fun: Fn(val![par]) -> Val,
{
    parse![self.parser].map(when! {
        move => self.function,
        mut => &mut self.function,
        ref => &self.function,
    })
}

#[parser_fn]
fn map_err<par, Fun, Err>(self: &MapErr<par, Fun>) -> err![par<Err>]
where
    for<'impl_move> Fun: FnOnce(err![par]) -> Err,
    for<'impl_mut> Fun: FnMut(err![par]) -> Err,
    for<'impl_ref> Fun: Fn(err![par]) -> Err,
{
    parse![self.parser].map_err(when! {
        move => self.function,
        mut => &mut self.function,
        ref => &self.function,
    })
}

#[parser_fn]
fn sel<par, Fun>(self: &Sel<par, Fun>) -> <par::Output as Select<Fun>>::Output
where
    par::Output: Select<Fun>,
{
    parse![self.parser].sel(&self.function)
}

#[parser_fn]
fn sel_err<par, Fun>(self: &SelErr<par, Fun>) -> <par::Output as SelectErr<Fun>>::Output
where
    par::Output: SelectErr<Fun>,
{
    parse![self.parser].sel_err(&self.function)
}
