use crate::parser::prelude::*;
use crate::output::prelude::*;
use crate::output::util::try_op;
use crate::input::prelude::*;

/// A marker for defining a response in case the equallity check fails
pub struct OrElse<Res, Err>(Res, std::marker::PhantomData<Err>);

/// A parser for checking equallity with a value
///
/// This `struct` is created by the [`Parser::eq`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Eq<Par, Val, Mod = (), const I: bool = false> {
    parser: Par,
    value: Val,
    mode: Mod,
}

/// A parser for checking inequallity with a value
///
/// This `struct` is created by the [`Parser::ne`] method on [`Parser`].
/// See its documentation for more.
pub type Ne<Par, Val, Mod = ()> = Eq<Par, Val, Mod, true>;

/// A parser for checking equallity with a value,
/// generating an error in case of failure
///
/// This `struct` is created by the [`Eq::or_else`] method on [`Eq`].
/// See its documentation for more.
pub type EqOrElse<Par, Val, Res, Err> = Eq<Par, Val, OrElse<Res, Err>>;

/// A parser for checking inequallity with a value
/// generating an error in case of failure
///
/// This `struct` is created by the [`Ne::or_else`] method on [`Ne`].
/// See its documentation for more.
pub type NeOrElse<Par, Val, Res, Err> = Ne<Par, Val, OrElse<Res, Err>>;

impl<Par, Val, const I: bool> Eq<Par, Val, (), I> {
    pub(crate) fn new(parser: Par, value: Val) -> Self
    where
        Par: Operator,
        Par::Response: ValueFunctor,
        <Par::Response as Response>::Value: PartialEq<Val>,
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

impl<Par, Out, Val> Operator for Eq<Par, Val>
where
    Par: Operator<Response = Out>,
    Out: Filterable,
    Out::Value: PartialEq<Val>,
{
    type Scanner = Par::Scanner;
    type Response = <Out as Filterable>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .parse_next(input)
            .filter_response(|v| *v == self.value)
    }
}

impl<Par, Out, Val, Res, Err> Operator for EqOrElse<Par, Val, Res, Err>
where
    Par: Operator<Response = Out>,
    Out: FilterableWithErr<Err>,
    Out::Value: PartialEq<Val>,
    Res: Fn() -> Err,
{
    type Scanner = Par::Scanner;
    type Response = <Out as FilterableWithErr<Err>>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .parse_next(input)
            .filter_response_or_else(|v| *v == self.value, &self.mode.0)
    }
}

impl<Par, Out, Val> Operator for Ne<Par, Val>
where
    Par: Operator<Response = Out>,
    Out: Filterable,
    Out::Value: PartialEq<Val>,
{
    type Scanner = Par::Scanner;
    type Response = <Out as Filterable>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .parse_next(input)
            .filter_response(|v| *v != self.value)
    }
}

impl<Par, Out, Val, Res, Err> Operator for NeOrElse<Par, Val, Res, Err>
where
    Par: Operator<Response = Out>,
    Out: FilterableWithErr<Err>,
    Out::Value: PartialEq<Val>,
    Res: Fn() -> Err,
{
    type Scanner = Par::Scanner;
    type Response = <Out as FilterableWithErr<Err>>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .parse_next(input)
            .filter_response_or_else(|v| *v != self.value, &self.mode.0)
    }
}
