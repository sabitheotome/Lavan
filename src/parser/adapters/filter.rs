use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::output::util::try_op;
use crate::parser::prelude::*;

// TODO: Documentation
pub struct OrElse<Res, Err>(Res, std::marker::PhantomData<Err>);

/// A parser for filtering through a predicate
///
/// This `struct` is created by the [`Parser::filter`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Filter<Par, Fun, Mod = (), const I: bool = false> {
    parser: Par,
    predicate: Fun,
    mode: Mod,
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
    pub(crate) fn new(parser: Par, function: Fun) -> Self
    where
        Par: Operator,
        Par::Response: ValueFunctor,
        Fun: Fn(&<Par::Response as Response>::Value) -> bool,
    {
        Self {
            parser,
            predicate: function,
            mode: (),
        }
    }

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

impl<Par, Out, Fun> Operator for Filter<Par, Fun>
where
    Par: Operator<Response = Out>,
    Out: Filterable,
    Fun: Fn(&Out::Value) -> bool,
{
    type Scanner = Par::Scanner;
    type Response = <Out as Filterable>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .parse_next(input)
            .filter_response(&self.predicate)
    }
}

impl<Par, Out, Fun, Res, Err> Operator for FilterOrElse<Par, Fun, Res, Err>
where
    Par: Operator<Response = Out>,
    Out: FilterableWithErr<Err>,
    Fun: Fn(&Out::Value) -> bool,
    Res: Fn() -> Err,
{
    type Scanner = Par::Scanner;
    type Response = <Out as FilterableWithErr<Err>>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .parse_next(input)
            .filter_response_or_else(&self.predicate, &self.mode.0)
    }
}

impl<Par, Out, Fun> Operator for FilterNot<Par, Fun>
where
    Par: Operator<Response = Out>,
    Out: Filterable,
    Fun: Fn(&Out::Value) -> bool,
{
    type Scanner = Par::Scanner;
    type Response = <Out as Filterable>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .parse_next(input)
            .filter_response(|v| !(self.predicate)(v))
    }
}

impl<Par, Out, Fun, Res, Err> Operator for FilterNotOrElse<Par, Fun, Res, Err>
where
    Par: Operator<Response = Out>,
    Out: FilterableWithErr<Err>,
    Fun: Fn(&Out::Value) -> bool,
    Res: Fn() -> Err,
{
    type Scanner = Par::Scanner;
    type Response = <Out as FilterableWithErr<Err>>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .parse_next(input)
            .filter_response_or_else(|v| !(self.predicate)(v), &self.mode.0)
    }
}
