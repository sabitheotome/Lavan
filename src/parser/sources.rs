use super::traits::{Parse, Parser};
use crate::{
    input::prelude::*,
    output::{prelude::Sure, traits::Response},
};
use std::marker::PhantomData;

pub use functions::*;

mod functions {
    use super::{adapters::*, *};

    // TODO: Documentation
    pub fn eoi<Str>() -> EOI<Str>
    where
        Str: Scanner,
    {
        EOI(PhantomData)
    }

    // TODO: Documentation
    pub fn any<Str>() -> Any<Str>
    where
        Str: Scanner,
    {
        Any(PhantomData)
    }

    // TODO: Documentation
    pub fn any_if<Str, Fun>(f: Fun) -> AnyIf<Str, Fun>
    where
        Str: Scanner,
        Fun: Fn(&Str::Item) -> bool,
    {
        AnyIf(PhantomData, f)
    }

    // TODO: Documentation
    pub fn any_eq<Str>(v: Str::Item) -> AnyEq<Str, Str::Item>
    where
        Str: Scanner,
    {
        AnyEq(PhantomData, v)
    }

    // TODO: Documentation
    pub fn any_ne<Str>(v: Str::Item) -> AnyNe<Str, Str::Item>
    where
        Str: Scanner,
    {
        AnyEq(PhantomData, v)
    }

    // TODO: Documentation
    pub fn take<'a, Str, Ref>(size: usize) -> Take<'a, Str>
    where
        Str: Scanner + ScannerSlice<Slice = &'a Ref>,
        Ref: 'a + std::ops::Index<std::ops::RangeTo<usize>> + ?Sized,
    {
        Take(size, PhantomData)
    }

    // TODO: Documentation
    pub fn func<'a, Fun, Str, Out>(f: Fun) -> Func<Fun, Str, Out>
    where
        Str: Scanner,
        Out: Response,
        Fun: Fn(&mut Str) -> Out,
    {
        Func(f, PhantomData)
    }

    // TODO: Documentation
    pub fn make<Str, T>() -> Make<Str, T>
    where
        T: Parse<Str>,
        Str: Scanner,
    {
        Make(PhantomData)
    }
}

pub(crate) mod adapters {
    use super::*;

    /// A parser for expecting the next token to be an **End of File**
    ///
    /// This `struct` is created by the [`eoi`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct EOI<Str>(pub(super) PhantomData<Str>);

    /// A parser for expectingany  kind of any token besides **End of File**
    ///
    /// This `struct` is created by the [`any`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    pub struct Any<Str>(pub(super) PhantomData<Str>);

    /// A parser for expecting any token that matches a predicate
    ///
    /// This `struct` is created by the [`any_if`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    pub struct AnyIf<Str, Fun>(pub(super) PhantomData<Str>, pub(super) Fun);

    /// A parser for expecting a token to be equal to the provided value
    ///
    /// This `struct` is created by the [`any_eq`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    pub struct AnyEq<Str, Item, const I: bool = false>(
        pub(super) PhantomData<Str>,
        pub(super) Item,
    );

    /// A parser for expecting a token to be not equal to the provided value
    ///
    /// This `struct` is created by the [`any_ne`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    pub type AnyNe<Str, Item> = AnyEq<Str, Item, true>;

    /// A parser for taking a provided amount of tokens,
    /// returning a stream slice starting from the current offset
    ///
    /// This `struct` is created by the [`take`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    pub struct Take<'a, Str>(pub(super) usize, pub(super) PhantomData<&'a Str>);

    /// TODO: Documentation
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    pub struct Make<Str, T>(pub(super) PhantomData<(Str, T)>);

    /// TODO: Documentation
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    pub struct Func<Fun, Str, Out>(pub(super) Fun, pub(super) PhantomData<(Str, Out)>);
}

mod impls {
    use super::{adapters::*, *};

    impl<Str> Parser for EOI<Str>
    where
        Str: Scanner,
    {
        type Input = Str;
        type Output = bool;

        fn next(&self, input: &mut Self::Input) -> Self::Output {
            input.next().is_none()
        }
    }

    impl<Str> Parser for Any<Str>
    where
        Str: Scanner,
    {
        type Input = Str;
        type Output = Option<Str::Item>;

        fn next(&self, input: &mut Self::Input) -> Self::Output {
            input.next()
        }
    }

    impl<Str, Fun> Parser for AnyIf<Str, Fun>
    where
        Str: Scanner,
        for<'a> Fun: Fn(&'a Str::Item) -> bool,
    {
        type Input = Str;
        type Output = Option<Str::Item>;

        fn next(&self, input: &mut Self::Input) -> Self::Output {
            let mut peekable = input.peekable();
            let peek = peekable.peek()?;
            ((self.1)(peek)).then_some(peekable.next()?)
        }
    }

    impl<Str> Parser for AnyEq<Str, Str::Item>
    where
        Str: Scanner,
        Str::Item: PartialEq,
    {
        type Input = Str;
        type Output = Option<Str::Item>;

        fn next(&self, input: &mut Self::Input) -> Self::Output {
            any_if(|v| *v == self.1).next(input)
        }
    }

    impl<Str> Parser for AnyNe<Str, Str::Item>
    where
        Str: Scanner,
        Str::Item: PartialEq,
    {
        type Input = Str;
        type Output = Option<Str::Item>;

        fn next(&self, input: &mut Self::Input) -> Self::Output {
            any_if(|v| *v != self.1).next(input)
        }
    }

    impl<'a, Str, Ref> Parser for Take<'a, Str>
    where
        Str: Scanner + ScannerSlice<Slice = &'a Ref>,
        Ref: 'a + ?Sized,
    {
        type Input = Str;
        type Output = Option<&'a Ref>;

        fn next(&self, input: &mut Self::Input) -> Self::Output {
            any()
                .discard()
                .repeat_min(self.0)
                .slice()
                .next(input)
        }
    }

    impl<Fun, Str, Out> Parser for Func<Fun, Str, Out>
    where
        Str: Scanner,
        Out: Response,
        Fun: Fn(&mut Str) -> Out,
    {
        type Input = Str;
        type Output = Out;

        fn next(&self, input: &mut Self::Input) -> Self::Output {
            (self.0)(input)
        }
    }

    impl<Str, T> Parser for Make<Str, T>
    where
        T: Parse<Str>,
        Str: Scanner,
    {
        type Input = Str;
        type Output = T::Output;

        fn next(&self, input: &mut Self::Input) -> Self::Output {
            T::parse(input)
        }
    }
}
