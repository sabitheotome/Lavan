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
    parse_str::ParseStr,
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
    /// use lavan::parser::traits::Parser;
    /// use lavan::parser::sources::any;
    ///
    /// let mut stream = ("Hello, World!", 0);
    /// let first_char = any().parse_stream(&mut stream);
    /// assert_eq!(first_char, Some('H'));
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
    fn map<Fun>(self, f: Fun) -> Map<Self, Fun>
    where
        Self: Sized,
        Self::Output: Mappable<Fun>,
    {
        Map::new(self, f)
    }

    // TODO: Documentation
    fn map_err<Fun>(self, f: Fun) -> MapErr<Self, Fun>
    where
        Self: Sized,
        Self::Output: ErrMappable<Fun>,
    {
        MapErr::new(self, f)
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
    fn ignore(self) -> Ignore<Self>
    where
        Self: Sized,
        Self::Output: Ignorable,
    {
        Ignore::new(self)
    }

    /// Automatically backtracks if the parsing has failed
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use lavan::parser::traits::Parser;
    /// use lavan::parser::sources::any_if;
    ///
    /// let input = "not a digit";
    /// let mut stream = (input, 0);
    /// let mut stream_auto = (input, 0);
    ///
    /// let parse_a_digit = any_if(|c: &char| c.is_ascii_digit());
    /// let parse_a_digit_auto = any_if(|c: &char| c.is_ascii_digit()).auto_bt();
    ///
    /// parse_a_digit.parse_stream(&mut stream);
    /// parse_a_digit_auto.parse_stream(&mut stream_auto);
    ///
    /// // WITHOUT AUTO_BT: Stream index is equal to 1
    /// assert_eq!(stream.1, 1);
    /// // WITH AUTO_BT: Stream index is equal to 0
    /// assert_eq!(stream_auto.1, 0);
    /// ```
    fn auto_bt(self) -> AutoBt<Self>
    where
        Self: Sized,
        Self::Output: Recoverable,
    {
        AutoBt::new(self)
    }

    // TODO: Documentation
    fn opt(self) -> Opt<Self>
    where
        Self: Sized,
        Self::Output: Optionable,
    {
        Opt::new(self)
    }

    // TODO: Documentation
    fn slice<'a>(self) -> Slice<'a, Self>
    where
        Self: Sized,
        Self::Input: StreamSlice<'a>,
        Self::Output: Response<Value = ()>,
    {
        Slice::new(self)
    }

    // TODO: Documentation
    fn filter<Fun>(self, f: Fun) -> Filter<Self, Fun>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        Fun: Fn(&<Self::Output as Response>::Value) -> bool,
    {
        Filter::new(self, f)
    }

    // TODO: Documentation
    fn filter_not<Fun>(self, f: Fun) -> FilterNot<Self, Fun>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        Fun: Fn(&<Self::Output as Response>::Value) -> bool,
    {
        self.filter(f).not()
    }

    // TODO: Documentation
    fn eq<Val>(self, v: Val) -> Eq<Self, Val>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        <Self::Output as Response>::Value: PartialEq<Val>,
    {
        Eq::new(self, v)
    }

    // TODO: Documentation
    fn ne<Val>(self, v: Val) -> Ne<Self, Val>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        <Self::Output as Response>::Value: PartialEq<Val>,
    {
        self.eq(v).not()
    }

    // TODO: Documentation
    fn parse_str<T>(self) -> ParseStr<Self, T>
    where
        Self: Sized,
        Self::Output: Bindable<fn(&str) -> Result<T, T::Err>>,
        T: std::str::FromStr,
    {
        ParseStr::new(self)
    }

    // TODO: Documentation
    fn and<Par>(self, parser: Par) -> And<Self, Par>
    where
        Self: Sized,
        Self::Output: Combinable<Par::Output>,
        Par: Parser<Input = Self::Input>,
    {
        And::new(self, parser)
    }

    // TODO: Documentation
    fn or<Par>(self, parser: Par) -> Or<Self, Par>
    where
        Self: Sized,
        Self::Output: Switchable<Par::Output>,
        Par: Parser<Input = Self::Input>,
    {
        Or::new(self, parser)
    }

    // TODO: Documentation
    fn repeat(self) -> Repeat<Self>
    where
        Self: Sized,
        Self::Output: Recoverable + Fallible,
    {
        Repeat::new(self, UntilErr(()))
    }

    // TODO: Documentation
    fn repeat_eoi(self) -> RepeatEOI<Self>
    where
        Self: Sized,
    {
        RepeatEOI::new(self, UntilEOI(()))
    }

    // TODO: Documentation
    // TODO: usize -> NonZeroUsize
    fn repeat_min(self, count: usize) -> RepeatMin<Self>
    where
        Self: Sized,
    {
        assert!(count >= 1);
        RepeatMin::new(self, Minimum(count))
    }

    // TODO: Documentation
    fn repeat_min_eoi(self, count: usize) -> RepeatMinEOI<Self>
    where
        Self: Sized,
    {
        assert!(count >= 1);
        RepeatMinEOI::new(self, MinimumEOI(count))
    }

    // TODO: Documentation
    fn repeat_max(self, count: usize) -> RepeatMax<Self>
    where
        Self: Sized,
        Self::Output: Fallible,
    {
        assert!(count >= 1);
        RepeatMax::new(self, Maximum(count))
    }

    // TODO: Documentation
    fn repeat_exact(self, count: usize) -> RepeatExact<Self>
    where
        Self: Sized,
    {
        assert!(count >= 1);
        RepeatExact::new(self, Exact(count))
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

    fn parse_stream(self, input: &mut Self::Input) -> Self::Output {
        (self.1)(self.0, input)
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
