use std::process::Output;

use super::sources::adapters::{Any, Func};
use super::util::assoc::{err, val};
use super::{
    adapters::{
        and::And,
        as_ref::AsRef,
        auto_bt::AutoBt,
        delimited::Delimited,
        eq::{Eq, Ne},
        filter::{Filter, FilterNot},
        ignore::Discard,
        map::Map,
        map_err::MapErr,
        ok::Ok,
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
    },
    sources::{any, func},
};
use crate::input::prelude::*;
use crate::output::prelude::*;

pub trait Parser<Input>
where
    Input: Scanner,
{
    type Operator: Operator<Scanner = Input, Response = Self::Output>;
    type Output: Response;

    fn operator(self) -> Self::Operator;

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
    fn evaluate(self, input: impl IntoScanner<IntoScanner = Input>) -> Self::Output
    where
        Self: Sized,
    {
        self.operator().parse_next(&mut input.into_scanner())
    }

    /// Take this parser by reference, without consuming it.
    /// This operation can be useful if you want to reuse the parser later.
    ///
    /// Since [parse_stream](Parser::parse_stream) takes `self` by reference,
    /// it is possible to plug adapters without losing ownership.
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "YesOrNo";
    ///
    /// let complex_parser = any(); // imagine this a very complex parser
    /// let super_complex_parser =
    ///     // you can use the same parser twice!
    ///     complex_parser.as_ref().eq('Y').or(
    ///         complex_parser.as_ref().eq('N')
    ///     );
    /// let yesOrNo = super_complex_parser.evaluate(input);
    /// assert_eq!(yesOrNo, Some('Y'));
    /// ```
    fn as_ref(&self) -> AsRef<'_, Self>
    where
        Self: Sized,
        Self: Operator,
        Self: Parser<Input, Operator = Self>,
    {
        AsRef::new(self)
    }

    /// Maps the response's [Value](Response::Value) to another type.
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "A";
    /// let is_uppercase: Option<bool> =
    ///     any().map(char::is_uppercase).evaluate(input);
    /// assert_eq!(is_uppercase, Some(true));
    /// ```
    fn map<Fun>(self, f: Fun) -> Map<Self::Operator, Fun>
    where
        Self: Sized,
        Self::Output: Mappable<Fun>,
    {
        Map::new(self.operator(), f)
    }

    /// Maps the [Error](Response::Error) contained in the
    /// [Output](Parser::Output) to another type.
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct ExpectedSomethingError;
    ///
    /// let input = "Nothing";
    /// let result: Result<&str,  ExpectedSomethingError> = word("Something")
    ///     .map_err(|| ExpectedSomethingError).evaluate(input);
    /// assert_eq!(result, Err(ExpectedSomethingError));
    /// ```
    fn map_err<Fun>(self, f: Fun) -> MapErr<Self::Operator, Fun>
    where
        Self: Sized,
        Self::Output: ErrMappable<Fun>,
    {
        MapErr::new(self.operator(), f)
    }

    /// Map the value contained in the [Output](Parser::Output) to a [Response].
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "A";
    /// let maybe_uppercase : Option<char> = any()
    ///     .then(|c: char| Some(c)
    ///         .filter(char::is_ascii_uppercase))
    ///     .evaluate(input);
    /// assert_eq!(maybe_uppercase, Some('A'));
    /// ```
    fn then<Fun>(self, f: Fun) -> Then<Self::Operator, Fun>
    where
        Self: Sized,
        Self::Output: Apply<Fun>,
    {
        Then::new(self.operator(), f)
    }

    // TODO: rename to "discard"
    /// Discards the response's [Value](Response::Value).
    /// This operation converts the [Output](Parser::Output) into a [`Attachable`]
    /// equivalent, defined by the [Ignorable] trait.
    ///
    /// This operation can be required by some operations, to check if the
    /// programmer is aware of the discartion. It can also be useful for
    /// convenience with mapping and type simplification.
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "Something";
    /// let is_all_alphabetic: bool = any_if(char::is_ascii_alphabetic)
    ///     .discard().repeat_eoi().evaluate(input);
    /// assert_eq!(is_all_alphabetic, true);
    /// ```
    fn discard(self) -> Discard<Self::Operator>
    where
        Self: Sized,
        Self::Output: Ignorable,
    {
        Discard::new(self.operator())
    }

    // TODO: Documentation
    fn ok(self) -> Ok<Self::Operator>
    where
        Self: Sized,
        Self::Output: ErrIgnorable,
    {
        Ok::new(self.operator())
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
    fn auto_bt(self) -> AutoBt<Self::Operator>
    where
        Self: Sized,
        Self::Output: Fallible,
    {
        AutoBt::new(self.operator())
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
    fn opt(self) -> Opt<Self::Operator>
    where
        Self: Sized,
        Self::Output: Fallible,
    {
        Opt::new(self.operator())
    }

    /// Yield a slice of the [Input](Parser::Input), defined by the startijng
    /// offset and the ending offset of the execution of the parser.
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "Hello, World!";
    /// let slice: Sure<&str> =
    ///     any().discard().repeat().slice().evaluate(input);
    /// assert_eq!(slice.value(), "Hello, World!");
    /// ```
    fn slice<'a>(self) -> Slice<'a, Self::Operator>
    where
        Self: Sized,
        Input: ScannerSlice,
        Self::Output: Attachable,
    {
        Slice::new(self.operator())
    }

    /// Filters the response's [Value](Response::Value) through a predicate.
    /// The [Output](Parser::Output) is a fallible version of the current response,
    /// defined by the [Filterable] and [FilterableWithErr] traits.
    ///
    /// It's possible to define a unmatch case through [or_else](Filter::or_else).
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "Lavan";
    /// let is_lavan: bool =
    ///     take(5).filter(|s| *s == "Lavan").discard().evaluate(input);
    /// assert_eq!(is_lavan, true);
    /// ```
    fn filter<Fun>(self, f: Fun) -> Filter<Self::Operator, Fun>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        Fun: Fn(&<Self::Output as Response>::Value) -> bool,
    {
        Filter::new(self.operator(), f)
    }

    /// Filters the response's [Value](Response::Value) through a inverted predicate.
    /// The [Output](Parser::Output) is a fallible version of the current response,
    /// defined by the [Filterable] and [FilterableWithErr] traits.
    ///
    /// For defining equallity conditions, check [filter](Parser::filter).
    /// It's possible to define a unmatch case through [or_else](Filter::or_else).
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "Lavan";
    /// let is_lavan: bool =
    ///     take(5).filter(|s| *s == "Lavan").discard().evaluate(input);
    /// assert_eq!(is_lavan, true);
    /// ```
    fn filter_not<Fun>(self, f: Fun) -> FilterNot<Self::Operator, Fun>
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
    /// It's possible to define a unmatch case through [or_else](Eq::or_else).
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "Lavan";
    /// let is_lavan: bool = take(5).eq("Lavan").discard().evaluate(input);
    /// assert_eq!(is_lavan, true);
    /// ```
    fn eq<Val>(self, v: Val) -> Eq<Self::Operator, Val>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        <Self::Output as Response>::Value: PartialEq<Val>,
    {
        Eq::new(self.operator(), v)
    }

    /// Make an inequallity condition for the response [Value](Response::Value).
    /// The [Output](Parser::Output) is a fallible version of the current response,
    /// defined by the [Filterable] and [FilterableWithErr] traits.
    ///
    /// For defining equallity conditions, check [eq](Parser::eq).
    /// It's possible to define a unmatch case through [or_else](Ne::or_else).
    ///
    /// # Examples
    /// Basic usage:
    ///```
    /// use lavan::prelude::*;
    ///
    /// let input = "Legal!!";
    /// let is_legal: bool = take(7).ne("Illegal").discard().evaluate(input);
    /// assert_eq!(is_legal, true);
    /// ```
    fn ne<Val>(self, v: Val) -> Ne<Self::Operator, Val>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        <Self::Output as Response>::Value: PartialEq<Val>,
    {
        self.eq(v).not()
    }

    // TODO: Documentation
    fn boxed<T>(self) -> super::adapters::map::FnMap<Self::Operator, T, Box<T>>
    where
        Self: Sized,
        Self::Output: Mappable<fn(T) -> Box<T>>,
    {
        self.map(Box::new)
    }

    /// Convert the value contained in the [Output](Parser::Output) into `T`
    /// where `T` implements [FromStr](std::str::FromStr).
    ///
    // TODO: # Examples
    // Basic usage:
    //```
    // use lavan::prelude::*;
    //
    // let input = "7";
    // let digit_to_u32 : Result<u32, ParseIntError> = any().slice()
    //     .parse_str::<u32>().evaluate(input);
    // assert_eq!(digit_to_u32, Some(7));
    // ```
    fn parse_str<T>(self) -> ParseStr<Self::Operator, T>
    where
        Self: Sized,
        Self::Output: Apply<fn(&str) -> Result<T, T::Err>>,
        T: std::str::FromStr,
    {
        ParseStr::new(self.operator())
    }

    // TODO: Documentation
    fn unwrapped(self) -> Unwrapped<Self::Operator>
    where
        Self: Sized,
        Self::Output: Fallible + ValueFunctor,
        <Self::Output as Response>::Error: std::fmt::Debug,
    {
        Unwrapped::new(self.operator())
    }

    // TODO: Documentation
    fn owned(self) -> Owned<Self::Operator>
    where
        Self: Sized,
        Self::Output: ValueFunctor,
        <Self::Output as Response>::Value: std::borrow::ToOwned,
    {
        Owned::new(self.operator())
    }

    // TODO: Documentation
    fn spanned(self) -> Spanned<Self::Operator>
    where
        Self: Sized,
        Input: ScannerSpan,
        Self::Output: ValueFunctor,
    {
        Spanned::new(self.operator())
    }

    // TODO: Documentation
    fn delimited<Del0, Del1, First, Second>(
        self,
        open: Del0,
        close: Del1,
    ) -> Delimited<Self::Operator, Del0, Del1>
    where
        Self: Sized,
        Del0: Parser<Input>,
        Del1: Parser<Input>,
        Del0::Output: Combine<Self::Output, Output = First>,
        First: Combine<Del1::Output, Output = Second>,
        Second: Response,
    {
        Delimited::new(self.operator(), open, close)
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
    fn and<Par>(self, parser: Par) -> And<Self::Operator, Par::Operator>
    where
        Self: Sized,
        Self::Output: Combine<Par::Output>,
        Par: Parser<Input>,
    {
        And::new(self.operator(), parser.operator())
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
    ///     any_eq('T').discard().map(|| Tails)
    ///     .or(any_eq('H').discard().map(|| Heads))
    ///     .either().evaluate(input).unwrap();
    /// assert_eq!(tailsOrHeads, Right(Heads));
    /// ```
    fn or<Par>(self, parser: Par) -> Or<Self::Operator, Par::Operator>
    where
        Self: Sized,
        //Self::Output: Switchable<<Par::Output as Response>::WithVal<<Self::Output as Response>::Value>>,
        Par: Parser<Input>,
    {
        Or::new(self.operator(), parser.operator())
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
    ///```
    fn try_with<Par, Fun, Out1>(
        self,
        parser: Par,
        function: Fun,
    ) -> TryWith<Self::Operator, Par::Operator, Fun>
    where
        Self: Sized,
        Par: Parser<Input, Output = Out1>,
        Fun: Fn(val![Self], Out1::Value) -> std::ops::ControlFlow<val![Self], val![Self]>,
        Out1: Response<Error = err![Self]>,
    {
        TryWith::new(self.operator(), parser.operator(), function)
    }

    // TODO: Documentation
    fn repeat(self) -> Repeat<Self::Operator>
    where
        Self: Sized,
        Self::Output: Fallible,
    {
        Repeat::new(self.operator(), UntilErr(()))
    }

    // TODO: Documentation
    fn repeat_eoi(self) -> RepeatEOI<Self::Operator>
    where
        Self: Sized,
    {
        RepeatEOI::new(self.operator(), UntilEOI(()))
    }

    // TODO: Documentation
    // TODO: usize -> NonZeroUsize
    fn repeat_min(self, count: usize) -> RepeatMin<Self::Operator>
    where
        Self: Sized,
    {
        assert!(count >= 1);
        RepeatMin::new(self.operator(), Minimum(count))
    }

    // TODO: Documentation
    fn repeat_min_eoi(self, count: usize) -> RepeatMinEOI<Self::Operator>
    where
        Self: Sized,
    {
        assert!(count >= 1);
        RepeatMinEOI::new(self.operator(), MinimumEOI(count))
    }

    // TODO: Documentation
    fn repeat_max(self, count: usize) -> RepeatMax<Self::Operator>
    where
        Self: Sized,
        Self::Output: Fallible,
    {
        assert!(count >= 1);
        RepeatMax::new(self.operator(), Maximum(count))
    }

    // TODO: Documentation
    fn repeat_exact(self, count: usize) -> RepeatExact<Self::Operator>
    where
        Self: Sized,
    {
        assert!(count >= 1);
        RepeatExact::new(self.operator(), Exact(count))
    }
}

impl<F, Input, Output> Parser<Input> for F
where
    F: Fn(&mut Input) -> Output,
    Input: Scanner,
    Output: Response,
{
    type Output = Output;
    type Operator = Func<Self, Input, Output>;

    fn operator(self) -> Self::Operator {
        func(self)
    }
}

pub trait Operator: Parser<Self::Scanner, Output = Self::Response, Operator = Self> {
    /// The input [`Scanner`] iterated by the parser
    type Scanner: Scanner;
    /// The output [`Response`] returned by the parser
    type Response: Response;

    /// Partially parses the referenced `input`, advancing the stream
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use lavan::prelude::*;
    /// use lavan::stream::traits::IntoStream;
    ///
    /// let mut stream = "Hello, World!".into_stream();
    /// let first_char = any().next(&mut stream);
    /// assert_eq!(first_char, Some('H'));
    /// ```
    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Output;
}

pub trait Parse<Input>
where
    Input: Scanner,
{
    type Output: ValueFunctor<Value = Self>;

    fn parse(input: &mut Input) -> Self::Output;
}

impl<Input, T> Parse<Input> for Option<T>
where
    Input: Scanner,
    T: Parse<Input>,
    T::Output: Fallible<Optional = Sure<Option<T>>>,
{
    type Output = Sure<Self>;

    fn parse(input: &mut Input) -> Self::Output {
        T::parse.opt().parse_next(input)
    }
}

#[cfg(feature = "either")]
impl<Input, L, R> Parse<Input> for either::Either<L, R>
where
    Input: Scanner,
    L: Parse<Input>,
    R: Parse<Input, Output = L::Output>,
    L::Output: Switch<R::Output>,

    val![L<either::Either<val![L], val![R]>>]:
        Switch<val![R<either::Either<val![L], val![R]>>], Output = Sure<either::Either<L, R>>>,
{
    type Output = Sure<Self>;

    fn parse(input: &mut Input) -> Self::Output {
        super::sources::make::<Input, L>()
            .or(super::sources::make::<Input, R>())
            .either()
            .parse_next(input)
    }
}
