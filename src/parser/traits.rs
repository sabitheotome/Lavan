use super::adapters::{
    core::and::And,
    core::as_ref::AsRef,
    core::filter::Filter,
    core::ignore::Ignore,
    core::map::Map,
    core::map_err::MapErr,
    core::non_terminal::NonTerminal,
    core::opt::Opt,
    core::or::Or,
    core::repeat::{mode::*, *},
    core::try_map::TryMap,
};
use super::util::assoc::{err, val};
use crate::response::prelude::*;
use crate::stream::traits::Stream;

pub trait Parser {
    type Input: Stream;
    type Output: Response;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output;

    fn as_ref(&self) -> AsRef<'_, Self>
    where
        Self: Sized,
    {
        AsRef::new(self)
    }

    fn map<Fun>(&self, f: Fun) -> Map<AsRef<Self>, Fun>
    where
        Self: Sized,
        Self::Output: Mappable<Fun>,
    {
        Map::new(self.as_ref(), f)
    }

    fn map_err<Fun>(&self, f: Fun) -> MapErr<AsRef<Self>, Fun>
    where
        Self: Sized,
        Self::Output: ErrMappable<Fun>,
    {
        MapErr::new(self.as_ref(), f)
    }

    fn try_map<Fun, Val>(&self, f: Fun) -> TryMap<AsRef<Self>, Fun, Val>
    where
        Self: Sized,
        Self::Output: Pseudodata + Exceptional,
        Fun: Fn(val![Self]) -> val![Self<Val>],
    {
        TryMap::new(self.as_ref(), f)
    }

    fn ignore(&self) -> Ignore<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Ignorable,
    {
        Ignore::new(self.as_ref())
    }

    fn non_terminal(&self) -> NonTerminal<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Recoverable,
    {
        NonTerminal::new(self.as_ref())
    }

    fn opt(&self) -> Opt<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Optionable + Iterator,
    {
        Opt::new(self.as_ref())
    }

    fn filter<Fun>(&self, function: Fun) -> Filter<AsRef<Self>, Fun>
    where
        Self: Sized,
        Self::Output: Triable,
        Fun: Fn(&<Self::Output as Pseudotriable>::Output) -> bool,
    {
        Filter::new(self.as_ref(), function)
    }

    fn and<Par>(&self, parser: Par) -> And<AsRef<Self>, Par>
    where
        Self: Sized,
        Self::Output: Combinable<Par::Output>,
        Par: Parser<Input = Self::Input>,
    {
        And::new(self.as_ref(), parser)
    }

    fn or<Par>(&self, parser: Par) -> Or<AsRef<Self>, Par>
    where
        Self: Sized,
        Self::Output: Disjoinable<Par::Output>,
        Par: Parser<Input = Self::Input>,
    {
        Or::new(self.as_ref(), parser)
    }

    fn repeat(&self) -> Repeat<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Recoverable + Triable,
    {
        Repeat::new(self.as_ref(), UntilErr(()))
    }

    fn repeat_eoi(&self) -> RepeatEOI<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Pseudotriable,
    {
        RepeatEOI::new(self.as_ref(), UntilEOI(()))
    }

    fn repeat_min(&self, count: usize) -> RepeatMin<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Pseudotriable,
    {
        assert!(count >= 1);
        RepeatMin::new(self.as_ref(), Minimum(count))
    }

    fn repeat_min_eoi(&self, count: usize) -> RepeatMinEOI<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Pseudotriable,
    {
        assert!(count >= 1);
        RepeatMinEOI::new(self.as_ref(), MinimumEOI(count))
    }

    fn repeat_max(&self, count: usize) -> RepeatMax<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Pseudotriable + Triable,
    {
        assert!(count >= 1);
        RepeatMax::new(self.as_ref(), Maximum(count))
    }

    fn repeat_exact(&self, count: usize) -> RepeatExact<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Pseudotriable,
    {
        assert!(count >= 1);
        RepeatExact::new(self.as_ref(), Exact(count))
    }
}

impl<Out: Response, Str: Stream> Parser for fn(&mut Str) -> Out {
    type Input = Str;
    type Output = Out;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self(input)
    }
}

pub trait Parse {
    type Input: Stream;

    fn parse(input: &mut Self::Input) -> Self;
}
