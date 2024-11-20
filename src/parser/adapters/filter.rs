use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

// TODO: Documentation
pub struct OrElse<Res, Err>(Res, std::marker::PhantomData<Err>);

/// A parser for filtering through a predicate
///
/// This `struct` is created by the [`Parser::filter`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Filter<Par, Fun, Mod = (), const I: bool = false> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) predicate: Fun,
    pub(in crate::parser) mode: Mod,
}

/// A parser for filtering through an inverted predicate
///
/// This `struct` is created by the [`Parser::filter_not`] method on [`Parser`].
/// See its documentation for more.
pub type FilterNot<Par, Fun, Mod = ()> = Filter<Par, Fun, Mod, true>;

// TODO: Documentation
pub type FilterOrElse<Par, Fun, Res, Err> = Filter<Par, Fun, OrElse<Res, Err>>;

// TODO: Documentation
pub type FilterNotOrElse<Par, Fun, Res, Err> = FilterNot<Par, Fun, OrElse<Res, Err>>;

impl<Par, Fun, const I: bool> Filter<Par, Fun, (), I> {
    pub fn or_else<Res, Err>(self, f: Res) -> Filter<Par, Fun, OrElse<Res, Err>, I>
    where
        Res: Fn() -> Err,
    {
        Filter {
            parser: self.parser,
            predicate: self.predicate,
            mode: OrElse(f, std::marker::PhantomData),
        }
    }
}

impl<Par, Fun, Mod> Filter<Par, Fun, Mod> {
    pub fn not(self) -> FilterNot<Par, Fun, Mod> {
        Filter {
            parser: self.parser,
            predicate: self.predicate,
            mode: self.mode,
        }
    }
}

#[parser_fn]
fn filter<par, Fun>(self: Filter<par, Fun>) -> <par::Output as Predict>::Output
where
    par::Output: Predict,
    Fun: Fn(&val![par::Output]) -> bool,
{
    parse![self.parser].predict(&self.predicate)
}

#[parser_fn]
fn filter_or_else<par, Fun, Res, Err>(
    self: FilterOrElse<par, Fun, Res, Err>,
) -> <par::Output as PredictOrElse<Err>>::Output
where
    par::Output: PredictOrElse<Err>,
    Fun: Fn(&val![par]) -> bool,
    Res: Fn() -> Err,
{
    parse![self.parser].predict_or_else(&self.predicate, &self.mode.0)
}

#[parser_fn]
fn filter_not<par, Fun>(self: FilterNot<par, Fun>) -> <par::Output as Predict>::Output
where
    par::Output: Predict,
    Fun: Fn(&val![par::Output]) -> bool,
{
    parse![self.parser].predict(|v| !(self.predicate)(v))
}

#[parser_fn]
fn filter_not_or_else<par, Fun, Res, Err>(
    self: FilterNotOrElse<par, Fun, Res, Err>,
) -> <par::Output as PredictOrElse<Err>>::Output
where
    par::Output: PredictOrElse<Err>,
    Fun: Fn(&val![par]) -> bool,
    Res: Fn() -> Err,
{
    parse![self.parser].predict_or_else(|v| !(self.predicate)(v), &self.mode.0)
}
