use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::response::util::try_op;
use crate::stream::traits::Stream;

// TODO: Documentation
pub struct OrElse<Res, Err>(Res, std::marker::PhantomData<Err>);

/// A parser for filtering through a predicate
///
/// This `struct` is created by the [`Parser::filter`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
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
        Par: Parser,
        Par::Output: ValueFunctor,
        Fun: Fn(&<Par::Output as Response>::Value) -> bool,
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

impl<Par, Out, Fun> Parser for Filter<Par, Fun>
where
    Par: Parser<Output = Out>,
    Out: Filterable,
    Fun: Fn(&Out::Value) -> bool,
{
    type Input = Par::Input;
    type Output = <Out as Filterable>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .parse_stream(input)
            .filter_response(&self.predicate)
    }
}

impl<Par, Out, Fun, Res, Err> Parser for FilterOrElse<Par, Fun, Res, Err>
where
    Par: Parser<Output = Out>,
    Out: FilterableWithErr<Err>,
    Fun: Fn(&Out::Value) -> bool,
    Res: Fn() -> Err,
{
    type Input = Par::Input;
    type Output = <Out as FilterableWithErr<Err>>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .parse_stream(input)
            .filter_response_or_else(&self.predicate, &self.mode.0)
    }
}

impl<Par, Out, Fun> Parser for FilterNot<Par, Fun>
where
    Par: Parser<Output = Out>,
    Out: Filterable,
    Fun: Fn(&Out::Value) -> bool,
{
    type Input = Par::Input;
    type Output = <Out as Filterable>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .parse_stream(input)
            .filter_response(|v| !(self.predicate)(v))
    }
}

impl<Par, Out, Fun, Res, Err> Parser for FilterNotOrElse<Par, Fun, Res, Err>
where
    Par: Parser<Output = Out>,
    Out: FilterableWithErr<Err>,
    Fun: Fn(&Out::Value) -> bool,
    Res: Fn() -> Err,
{
    type Input = Par::Input;
    type Output = <Out as FilterableWithErr<Err>>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .parse_stream(input)
            .filter_response_or_else(|v| !(self.predicate)(v), &self.mode.0)
    }
}
