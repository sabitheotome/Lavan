use crate::parser::prelude::internal::*;

/// A parser for checking equallity with a value
///
/// This `struct` is created by the [`Parser::eq`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Eq<Par, Val, Mod = (), const I: bool = false> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) value: Val,
    pub(in crate::parser) mode: Mod,
}

#[parser_fn]
fn eq<par, Val>(self: &Eq<par, Val>) -> <par::Output as Predict>::Output
where
    par::Output: Predict,
    val![par::Output]: PartialEq<Val>,
{
    parse![self.parser].predict(|v| *v == self.value)
}

/// A parser for checking equallity with a value,
/// generating an error in case of failure
///
/// This `struct` is created by the [`Eq::or_else`] method on [`Eq`].
/// See its documentation for more.
pub type EqOrElse<Par, Val, Res, Err> = Eq<Par, Val, OrElse<Res, Err>>;

#[parser_fn]
fn eq_or_else<par, Val, Res, Err>(
    self: &EqOrElse<par, Val, Res, Err>,
) -> <par::Output as PredictOrElse<Err>>::Output
where
    par::Output: PredictOrElse<Err>,
    val![par::Output]: PartialEq<Val>,
    Res: Fn() -> Err,
{
    parse![self.parser].predict_or_else(|v| *v == self.value, &self.mode.0)
}

/// A parser for checking inequallity with a value
///
/// This `struct` is created by the [`Parser::ne`] method on [`Parser`].
/// See its documentation for more.
pub type Ne<Par, Val, Mod = ()> = Eq<Par, Val, Mod, true>;

#[parser_fn]
fn ne<par, Val>(self: &Ne<par, Val>) -> <par::Output as Predict>::Output
where
    par::Output: Predict,
    val![par::Output]: PartialEq<Val>,
{
    parse![self.parser].predict(|v| *v != self.value)
}

/// A parser for checking inequallity with a value
/// generating an error in case of failure
///
/// This `struct` is created by the [`Ne::or_else`] method on [`Ne`].
/// See its documentation for more.
pub type NeOrElse<Par, Val, Res, Err> = Ne<Par, Val, OrElse<Res, Err>>;

#[parser_fn]
fn ne_or_else<par, Val, Res, Err>(
    self: &NeOrElse<par, Val, Res, Err>,
) -> <par::Output as PredictOrElse<Err>>::Output
where
    par::Output: PredictOrElse<Err>,
    val![par::Output]: PartialEq<Val>,
    Res: Fn() -> Err,
{
    parse![self.parser].predict_or_else(|v| *v != self.value, &self.mode.0)
}

/// A marker for defining a response in case the equallity check fails
pub struct OrElse<Res, Err>(Res, std::marker::PhantomData<Err>);

impl<Par, Val, const I: bool> Eq<Par, Val, (), I> {
    pub(crate) fn new<Input>(parser: Par, value: Val) -> Self
    where
        Par: ParseOnce<Input>,
        Par::Output: ValueResponse,
        <Par::Output as Response>::Value: PartialEq<Val>,
    {
        Self {
            parser,
            value,
            mode: (),
        }
    }

    /// TODO: Documentation
    pub fn or_else<Res, Err>(self, f: Res) -> Eq<Par, Val, OrElse<Res, Err>, I>
    where
        Res: Fn() -> Err,
    {
        Eq {
            parser: self.parser,
            value: self.value,
            mode: OrElse(f, std::marker::PhantomData),
        }
    }
}

impl<Par, Val, Mod> Eq<Par, Val, Mod> {
    /// TODO: Documentation
    pub fn not(self) -> Ne<Par, Val, Mod> {
        Eq {
            parser: self.parser,
            value: self.value,
            mode: self.mode,
        }
    }
}
