use super::adapters::{
    and::And,
    as_ref::AsRef,
    auto_bt::AutoBt,
    delimited::Delimited,
    eq::{Eq, Ne},
    filter::{Filter, FilterNot},
    ignore::Ignore,
    map::Map,
    map_err::MapErr,
    opt::Opt,
    or::Or,
    owned::Owned,
    parse_str::ParseStr,
    repeat::{mode::*, *},
    slice::Slice,
    spanned::Spanned,
    then::Then,
    try_with::TryWith,
    unwrapped::Unwrapped,
};
use super::util::assoc::{err, val};
use crate::stream::traits::{IntoStream, Stream};
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
    /// use lavan::prelude::*;
    /// use lavan::stream::traits::IntoStream;
    ///
    /// let mut stream = "Hello, World!".into_stream();
    /// let first_char = any().parse_stream(&mut stream);
    /// assert_eq!(first_char, Some('H'));
    /// ```
    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output;

    /// Parses the token sequence `input`
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use lavan::prelude::*;
    ///
    /// let mut input = "Hello, World!";
    /// let first_char = any().evaluate(input);
    /// assert_eq!(first_char, Some('H'));
    /// ```
    fn evaluate(&self, input: impl IntoStream<Stream = Self::Input>) -> Self::Output {
        self.parse_stream(&mut input.into_stream())
    }

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
    /// use lavan::prelude::*;
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

    /// Make a fallible [Output](Parser::Output) response into a infallible response.
    /// This operation is similar to [auto_bt](Parser::auto_bt), automatically backtracking
    /// in case of fail. The difference lies on the type of [Response] used. Fallible responses
    /// short-circuit the parsing operation. Using this parser, however, prevents this short-circuiting
    /// behaviour, making it suitable for optional fields in a Abstract Syntax Tree.
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "ABABAB";
    /// let bees: Vec<Option<char>> =
    ///     any_eq('B').opt().repeat_exact(6)
    ///     .to_vec().evaluate(input).value();
    /// assert_eq!(bees.len(), 6);
    /// ```
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

    /// Make an equallity condition for the response [Value](Response::Value).
    /// The [Output](Parser::Output) is a fallible version of the current response,
    /// defined by the [Filterable] and [FilterableWithErr] traits.
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "Lavan";
    /// let is_lavan: bool = take(5).eq("Lavan").ignore().evaluate(input);
    /// assert_eq!(is_lavan, true);
    /// ```
    fn eq<Val>(self, v: Val) -> Eq<Self, Val>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        <Self::Output as Response>::Value: PartialEq<Val>,
    {
        Eq::new(self, v)
    }

    /// Make an inequallity condition for the response [Value](Response::Value).
    /// The [Output](Parser::Output) is a fallible version of the current response,
    /// defined by the [Filterable] and [FilterableWithErr] traits.
    ///
    /// For defining equallity conditions, check [eq](Parser::eq).
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "Legal!!";
    /// let is_legal: bool = take(7).ne("Illegal").ignore().evaluate(input);
    /// assert_eq!(is_legal, true);
    /// ```
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
    fn unwrapped(self) -> Unwrapped<Self>
    where
        Self: Sized,
        Self::Output: Fallible + ValueFunctor,
        <Self::Output as Response>::Error: std::fmt::Debug,
    {
        Unwrapped::new(self)
    }

    // TODO: Documentation
    fn owned(self) -> Owned<Self>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        <Self::Output as Response>::Value: std::borrow::ToOwned,
    {
        Owned::new(self)
    }

    // TODO: Documentation
    fn spanned(self) -> Spanned<Self>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
    {
        Spanned::new(self)
    }

    // TODO: Documentation
    fn delimited<Del, First, Second>(self, open: Del, close: Del) -> Delimited<Self, Del>
    where
        Self: Sized,
        Self::Input: Stream<Item = Del>,
        Del: PartialEq,
        Option<Del>: Combinable<Self::Output, Output = First>,
        First: Combinable<Option<Del>, Output = Second>,
        Second: Response,
    {
        Delimited::new(self, open, close)
    }

    /// Combine two parsers, running them subsequently.
    /// The output will be the combination of the two outputs,
    /// defined by the `trait` [Combinable].
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "ABC";
    /// let abc: Option<((char, char), char)> =
    ///     any().and(any()).and(any()).evaluate(input);
    /// assert_eq!(abc, Some((('A', 'B'), 'C')));
    /// ```
    fn and<Par>(self, parser: Par) -> And<Self, Par>
    where
        Self: Sized,
        Self::Output: Combinable<Par::Output>,
        Par: Parser<Input = Self::Input>,
    {
        And::new(self, parser)
    }

    /// Define a fallback parser in case this fails. Essentially, it
    /// will first attempt to run the first parser. If it fails, it will
    /// automatically backtrack, and the second parser will run.
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "H";
    /// let tailsOrHeads = any_eq('T')
    ///     .or(any_eq('H'))
    ///     .evaluate(input);
    /// assert_eq!(tailsOrHeads, Some('H'));
    /// ```
    /// Note: the two parsers need to have the same [Output](Parser::Output).
    /// If you want to have a union of two different outputs, consider using
    /// [either](Or::either) after the call of this function.
    ///```
    /// #![cfg(feature = "either")]
    /// use either::{Either, Either::*};
    /// use lavan::prelude::*;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Tails;
    /// #[derive(Debug, PartialEq)]
    /// struct Heads;
    ///
    /// let input = "H";
    /// let tailsOrHeads: Either<Tails, Heads> =
    ///     any_eq('T').ignore().map(|| Tails)
    ///     .or(any_eq('H').ignore().map(|| Heads))
    ///     .either().evaluate(input).unwrap();
    /// assert_eq!(tailsOrHeads, Right(Heads));
    /// ```
    fn or<Par>(self, parser: Par) -> Or<Self, Par>
    where
        Self: Sized,
        Self::Output:
            Switchable<<Par::Output as Response>::WithVal<<Self::Output as Response>::Value>>,
        Par: Parser<Input = Self::Input>,
    {
        Or::new(self, parser)
    }

    /// Try making a variant with another parser. Essentially, it tries
    /// to run the second parser, automatically backtracking in case of
    /// failure. After that, the provided closure will run, allowing for
    /// condition checking and conversions.
    ///
    /// Exiting the closure with [Continue][std::ops::ControlFlow::Continue]
    /// will consume all tokens.
    ///
    /// However, exiting the closure with [Break][std::ops::ControlFlow::Break]
    /// will backtrack to the moment before the execution of the second parser.
    ///
    /// This operation can be useful if you want to make left-recursive variants
    /// of your structure.
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    /// use HatOrNoHat::*;
    ///
    /// #[derive(PartialEq, Debug, Clone)]
    /// enum HatOrNoHat {
    ///     No,
    ///     Hat,
    ///     NoHat,
    /// }
    ///
    /// let input: &[HatOrNoHat] = &[Hat, Hat, No, Hat, Hat, No, Hat, No, Hat, Hat, No];
    /// let expected_out = [Hat, Hat, NoHat, Hat, NoHat, NoHat, Hat, No];
    ///
    /// let output = any()
    ///     .try_with(any(), |a: HatOrNoHat, b: HatOrNoHat| {
    ///         if a == No && b == Hat {
    ///             std::ops::ControlFlow::Continue(NoHat)
    ///         } else {
    ///             std::ops::ControlFlow::Break(a)
    ///         }
    ///     })
    ///     .repeat()
    ///     .to_vec()
    ///     .evaluate(input);
    /// assert_eq!(output.value(), expected_out);
    //```
    fn try_with<Par, Fun, Out0, Out1>(self, parser: Par, function: Fun) -> TryWith<Self, Par, Fun>
    where
        Self: Sized + Parser<Output = Out0>,
        Par: Parser<Output = Out1, Input = Self::Input>,
        Fun: Fn(Out0::Value, Out1::Value) -> std::ops::ControlFlow<Out0::Value, Out0::Value>,
        Out0: Response,
        Out1: Response<Error = Out0::Error>,
    {
        TryWith::new(self, parser, function)
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

impl<Out: Response, Str: Stream, T> Parser for (T, fn(&T, &mut Str) -> Out) {
    type Input = Str;
    type Output = Out;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        (self.1)(&self.0, input)
    }
}

impl<Par: Parser> Parser for fn() -> Par {
    type Input = Par::Input;
    type Output = Par::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self().parse_stream(input)
    }
}

pub trait Parse {
    type Input: Stream;
    type Output: ValueFunctor<Value = Self>;

    fn parse(input: &mut Self::Input) -> Self::Output;
}

// TODO: highly experimental
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
}

#[cfg(feature = "either")]
impl<L, R> Parse for either::Either<L, R>
where
    L: Parse,
    R: Parse<Input = L::Input, Output = L::Output>,
    L::Input: Stream,
    L::Output: Switchable<L::Output, Output = Sure<either::Either<L, R>>>,
{
    type Input = L::Input;
    type Output = Sure<Self>;

    fn parse(input: &mut Self::Input) -> Self::Output {
        L::parse.or(R::parse).either().parse_stream(input)
    }
}*/
