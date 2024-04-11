use super::adapters::{
    and::And,
    as_ref::AsRef,
    auto_bt::AutoBt,
    eq::{Eq, Ne},
    filter::{Filter, FilterNot},
    ignore::Ignore,
    map::Map,
    map_err::MapErr,
    opt::Opt,
    or::Or,
    repeat::{mode::*, *},
    slice::Slice,
    then::Then,
};
use super::util::assoc::{err, val};
use crate::stream::traits::Stream;
use crate::{response::prelude::*, stream::traits::StreamSlice};

pub trait Parser {
    /// The input [`Stream`] iterated by the parser
    type Input: Stream;
    /// The output [`Response`] returned by the parser
    type Output: Response;

    /// Parsers the referenced `input`, advancing the stream
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// let stream = ("Hello, World!", 0);
    /// let first_char = any().parse_stream(&mut stream);
    /// assert_eq!(first_char, "H");
    /// ```
    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output;

    // TODO: Documentation
    fn as_ref(&self) -> AsRef<'_, Self>
    where
        Self: Sized,
    {
        AsRef::new(self)
    }

    // TODO: Documentation
    fn map<Fun>(&self, f: Fun) -> Map<AsRef<Self>, Fun>
    where
        Self: Sized,
        Self::Output: Mappable<Fun>,
    {
        Map::new(self.as_ref(), f)
    }

    // TODO: Documentation
    fn map_err<Fun>(&self, f: Fun) -> MapErr<AsRef<Self>, Fun>
    where
        Self: Sized,
        Self::Output: ErrMappable<Fun>,
    {
        MapErr::new(self.as_ref(), f)
    }

    // TODO: Documentation
    fn then<Fun>(self, f: Fun) -> Then<Self, Fun>
    where
        Self: Sized,
        Self::Output: Bindable<Fun>,
    {
        Then::new(self, f)
    }

    // TODO: Documentation
    fn ignore(&self) -> Ignore<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Ignorable,
    {
        Ignore::new(self.as_ref())
    }

    fn auto_bt(&self) -> AutoBt<AsRef<Self>>
    /// Automatically backtracks if the parsing has failed
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// let input = "123!"
    /// let mut not_auto_stream = (input, 0);
    /// let mut auto_stream = (input, 0);
    ///
    /// let not_auto = any_if(|c: char| c.is_ascii_digit());
    /// let auto = not_auto.as_ref().auto_bt();
    ///
    /// not_auto.parse_stream(&mut not_auto_stream);
    /// // Stream index is not equal to 0
    /// assert_ne!(not_auto_stream.1, 0);
    ///     
    /// auto.parse_stream(&mut auto_stream);
    /// // Stream index is equal to 0
    /// assert_eq!(auto_stream.1, 0);
    /// ```
    where
        Self: Sized,
        Self::Output: Recoverable,
    {
        AutoBt::new(self.as_ref())
    }

    // TODO: Documentation
    fn opt(&self) -> Opt<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Optionable,
    {
        Opt::new(self.as_ref())
    }

    // TODO: Documentation
    fn slice<'a>(&self) -> Slice<'a, AsRef<Self>>
    where
        Self: Sized,
        Self::Input: StreamSlice<'a>,
        Self::Output: Response<Value = ()>,
    {
        Slice::new(self.as_ref())
    }

    // TODO: Documentation
    fn filter<Fun>(&self, f: Fun) -> Filter<AsRef<Self>, Fun>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        Fun: Fn(&<Self::Output as Response>::Value) -> bool,
    {
        Filter::new(self.as_ref(), f)
    }

    // TODO: Documentation
    fn filter_not<Fun>(&self, f: Fun) -> FilterNot<AsRef<Self>, Fun>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        Fun: Fn(&<Self::Output as Response>::Value) -> bool,
    {
        self.filter(f).not()
    }

    // TODO: Documentation
    fn eq<Val>(&self, v: Val) -> Eq<AsRef<Self>, Val>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        <Self::Output as Response>::Value: PartialEq<Val>,
    {
        Eq::new(self.as_ref(), v)
    }

    // TODO: Documentation
    fn ne<Val>(&self, v: Val) -> Ne<AsRef<Self>, Val>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        <Self::Output as Response>::Value: PartialEq<Val>,
    {
        self.eq(v).not()
    }

    // TODO: Documentation
    fn and<Par>(&self, parser: Par) -> And<AsRef<Self>, Par>
    where
        Self: Sized,
        Self::Output: Combinable<Par::Output>,
        Par: Parser<Input = Self::Input>,
    {
        And::new(self.as_ref(), parser)
    }

    // TODO: Documentation
    fn or<Par>(&self, parser: Par) -> Or<AsRef<Self>, Par>
    where
        Self: Sized,
        Self::Output: Switchable<Par::Output>,
        Par: Parser<Input = Self::Input>,
    {
        Or::new(self.as_ref(), parser)
    }

    // TODO: Documentation
    fn repeat(&self) -> Repeat<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Recoverable + Fallible,
    {
        Repeat::new(self.as_ref(), UntilErr(()))
    }

    // TODO: Documentation
    fn repeat_eoi(&self) -> RepeatEOI<AsRef<Self>>
    where
        Self: Sized,
    {
        RepeatEOI::new(self.as_ref(), UntilEOI(()))
    }

    // TODO: Documentation
    // TODO: usize -> NonZeroUsize
    fn repeat_min(&self, count: usize) -> RepeatMin<AsRef<Self>>
    where
        Self: Sized,
    {
        assert!(count >= 1);
        RepeatMin::new(self.as_ref(), Minimum(count))
    }

    // TODO: Documentation
    fn repeat_min_eoi(&self, count: usize) -> RepeatMinEOI<AsRef<Self>>
    where
        Self: Sized,
    {
        assert!(count >= 1);
        RepeatMinEOI::new(self.as_ref(), MinimumEOI(count))
    }

    // TODO: Documentation
    fn repeat_max(&self, count: usize) -> RepeatMax<AsRef<Self>>
    where
        Self: Sized,
        Self::Output: Fallible,
    {
        assert!(count >= 1);
        RepeatMax::new(self.as_ref(), Maximum(count))
    }

    // TODO: Documentation
    fn repeat_exact(&self, count: usize) -> RepeatExact<AsRef<Self>>
    where
        Self: Sized,
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

// TODO: Experimental
/*impl<Out: Response, Str: Stream, T> Parser for (T, fn(&T, &mut Str) -> Out) {
    type Input = Str;
    type Output = Out;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        (self.1)(&self.0, input)
    }
}*/

pub trait Parse {
    type Input: Stream;
    type Output: ValueFunctor<Value = Self>;

    fn parse(input: &mut Self::Input) -> Self::Output;
}

// TODO: Experimental
/*impl<T> Parse for Option<T>
where
    T: Parse,
    T::Output: Optionable<Output = Sure<Option<T>>>,
{
    type Input = T::Input;
    type Output = Sure<Self>;

    fn parse(input: &mut Self::Input) -> Self::Output {
        T::parse(input).opt_response()
    }
}*/
