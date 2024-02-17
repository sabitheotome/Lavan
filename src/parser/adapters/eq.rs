use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::response::util::try_op;
use crate::stream::traits::Stream;

pub struct OrElse<Res, Err>(Res, std::marker::PhantomData<Err>);

pub struct Eq<Par, Val, Mod = (), const I: bool = false> {
    parser: Par,
    value: Val,
    mode: Mod,
}

pub type Ne<Par, Val, Mod = ()> = Eq<Par, Val, Mod, true>;
pub type EqOrElse<Par, Val, Res, Err> = Eq<Par, Val, OrElse<Res, Err>>;
pub type NeOrElse<Par, Val, Res, Err> = Ne<Par, Val, OrElse<Res, Err>>;

impl<Par, Val, const I: bool> Eq<Par, Val, (), I> {
    pub(crate) fn new(parser: Par, value: Val) -> Self
    where
        Par: Parser,
        Par::Output: ValueFunctor,
        <Par::Output as Response>::Value: PartialEq<Val>,
    {
        Self {
            parser,
            value,
            mode: (),
        }
    }

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
    pub fn not(self) -> Ne<Par, Val, Mod> {
        Eq {
            parser: self.parser,
            value: self.value,
            mode: self.mode,
        }
    }
}

impl<Par, Out, Val> Parser for Eq<Par, Val>
where
    Par: Parser<Output = Out>,
    Out: Filterable,
    Out::Value: PartialEq<Val>,
{
    type Input = Par::Input;
    type Output = <Out as Filterable>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .parse_stream(input)
            .filter_response(|v| *v == self.value)
    }
}

impl<Par, Out, Val, Res, Err> Parser for EqOrElse<Par, Val, Res, Err>
where
    Par: Parser<Output = Out>,
    Out: FilterableWithErr<Err>,
    Out::Value: PartialEq<Val>,
    Res: Fn() -> Err,
{
    type Input = Par::Input;
    type Output = <Out as FilterableWithErr<Err>>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .parse_stream(input)
            .filter_response_or_else(|v| *v == self.value, &self.mode.0)
    }
}

impl<Par, Out, Val> Parser for Ne<Par, Val>
where
    Par: Parser<Output = Out>,
    Out: Filterable,
    Out::Value: PartialEq<Val>,
{
    type Input = Par::Input;
    type Output = <Out as Filterable>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .parse_stream(input)
            .filter_response(|v| *v != self.value)
    }
}

impl<Par, Out, Val, Res, Err> Parser for NeOrElse<Par, Val, Res, Err>
where
    Par: Parser<Output = Out>,
    Out: FilterableWithErr<Err>,
    Out::Value: PartialEq<Val>,
    Res: Fn() -> Err,
{
    type Input = Par::Input;
    type Output = <Out as FilterableWithErr<Err>>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .parse_stream(input)
            .filter_response_or_else(|v| *v != self.value, &self.mode.0)
    }
}
