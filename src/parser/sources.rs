use super::{super::input::prelude::*, super::output::prelude::*, prelude::*};
use std::marker::PhantomData;

pub use functions::*;

mod functions {
    use super::{adapters::*, *};
    use crate::parser::adapters::infer::Infer;

    // TODO: Documentation
    pub fn nop<Str>() -> Infer<Str, NOP>
    where
        Str: Scanner,
    {
        NOP.infer()
    }

    // TODO: Documentation
    pub fn eoi<Str>() -> Infer<Str, EOI>
    where
        Str: Scanner,
    {
        EOI.infer()
    }

    // TODO: Documentation
    pub fn any<Str>() -> Infer<Str, Any>
    where
        Str: Scanner,
    {
        Any.infer()
    }

    // TODO: Documentation
    pub fn any_if<Str, Fun>(f: Fun) -> Infer<Str, AnyIf<Fun>>
    where
        Str: Scanner,
        Fun: Fn(&Str::Item) -> bool,
    {
        AnyIf(f).infer()
    }

    // TODO: Documentation
    pub fn any_eq<Str>(v: Str::Item) -> Infer<Str, AnyEq<Str::Item>>
    where
        Str: Scanner,
        Str::Item: PartialEq,
    {
        AnyEq(v).infer()
    }

    // TODO: Documentation
    pub fn any_ne<Str>(v: Str::Item) -> Infer<Str, AnyNe<Str::Item>>
    where
        Str: Scanner,
        Str::Item: PartialEq,
    {
        AnyEq(v).infer()
    }

    // TODO: Documentation
    pub fn take<'a, Str, Ref>(size: usize) -> Infer<Str, Take<'a>>
    where
        Str: Scanner + ScannerSlice<Slice = &'a Ref>,
        Ref: 'a + std::ops::Index<std::ops::RangeTo<usize>> + ?Sized,
    {
        Take(size, PhantomData).infer()
    }

    // TODO: Documentation
    pub fn func<'a, Fun, Str, Out>(f: Fun) -> Infer<Str, Func<Fun, Out>>
    where
        Str: Scanner,
        Out: Response,
        Fun: FnOnce(&mut Str) -> Out,
    {
        Func(f, PhantomData).infer()
    }

    // TODO: Documentation
    pub fn mk<Str, T>() -> Infer<Str, Mk<T>>
    where
        T: Parse<Str>,
        Str: Scanner,
    {
        Mk(PhantomData).infer()
    }

    // TODO: Documentation
    pub fn mkdef<Str, T>() -> Infer<Str, MkDefault<T>>
    where
        T: Default,
        Str: Scanner,
    {
        MkDefault(PhantomData).infer()
    }

    // TODO: Documentation
    pub fn supply<Str, T>(v: T) -> Infer<Str, Supply<T>>
    where
        Str: Scanner,
    {
        Supply(v).infer()
    }
}

pub mod adapters {
    use super::*;

    /// A parser for expecting the next token to be an **End of File**
    ///
    /// This `struct` is created by the [`eoi`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy)]
    pub struct NOP;

    /// A parser for expecting the next token to be an **End of File**
    ///
    /// This `struct` is created by the [`eoi`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy)]
    pub struct EOI;

    /// A parser for expectingany  kind of any token besides **End of File**
    ///
    /// This `struct` is created by the [`any`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy)]
    pub struct Any;

    /// A parser for expecting any token that matches a predicate
    ///
    /// This `struct` is created by the [`any_if`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct AnyIf<Fun>(pub(crate) Fun);

    /// A parser for expecting a token to be equal to the provided value
    ///
    /// This `struct` is created by the [`any_eq`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct AnyEq<Item, const I: bool = false>(pub(crate) Item);

    /// A parser for expecting a token to be not equal to the provided value
    ///
    /// This `struct` is created by the [`any_ne`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    pub type AnyNe<Item> = AnyEq<Item, true>;

    /// A parser for taking a provided amount of tokens,
    /// returning a stream slice starting from the current offset
    ///
    /// This `struct` is created by the [`take`] method on [`sources`](crate::parser::sources).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct Take<'a>(pub(crate) usize, pub(crate) PhantomData<&'a ()>);

    /// TODO: Documentation
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct Func<Fun, Out>(pub(crate) Fun, pub(crate) PhantomData<Out>);

    /// TODO: Documentation
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy)]
    pub struct Mk<T>(pub(crate) PhantomData<T>);

    /// TODO: Documentation
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct MkDefault<T>(pub(crate) PhantomData<T>);

    /// TODO: Documentation
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct Supply<T>(pub(crate) T);
}

mod impls {
    use super::{adapters::*, *};

    #[parser_fn]
    fn nop(self: &NOP) -> () {}

    #[parser_fn]
    fn eoi(self: &EOI) -> bool {
        input.next().is_none()
    }

    #[parser_fn]
    fn any(self: &Any) -> Option<INPUT::Item> {
        input.next()
    }

    #[parser_fn]
    fn any_if<Fun>(self: &AnyIf<Fun>) -> Option<INPUT::Item>
    where
        for<'a> Fun: Fn(&'a INPUT::Item) -> bool,
    {
        let mut peekable = input.peekable();
        let peek = peekable.peek()?;
        ((self.0)(peek)).then_some(peekable.next()?)
    }

    #[parser_fn]
    fn any_eq(self: &AnyEq<INPUT::Item>) -> Option<INPUT::Item>
    where
        INPUT::Item: PartialEq,
    {
        parse![any_if(|v| *v == self.0)]
    }

    #[parser_fn]
    fn any_ne(self: &AnyNe<INPUT::Item>) -> Option<INPUT::Item>
    where
        INPUT::Item: PartialEq,
    {
        parse![any_if(|v| *v != self.0)]
    }

    #[parser_fn]
    fn take<'a, Ref>(self: &Take<'a>) -> Option<&'a Ref>
    where
        INPUT: Scanner + ScannerSlice<Slice = &'a Ref>,
        Ref: 'a + ?Sized,
    {
        //any().discard().repeat_min(self.0).slice().parse_next(input)
        todo!()
    }

    #[parser_fn]
    fn func<Fun, Out>(self: &Func<Fun, Out>) -> Out
    where
        Out: Response,
        for<'once> Fun: FnOnce(&mut INPUT) -> Out,
        for<'mut> Fun: FnMut(&mut INPUT) -> Out,
        for<'const> Fun: Fn(&mut INPUT) -> Out,
    {
        (self.0)(input)
    }

    #[parser_fn]
    fn mk<T>(self: &Mk<T>) -> T::Output
    where
        T: Parse<INPUT>,
    {
        T::parse(input)
    }

    #[parser_fn]
    fn mk_default<T>(self: &MkDefault<T>) -> Sure<T>
    where
        T: Default,
    {
        Sure(T::default())
    }

    #[parser_fn]
    fn supply<T>(self: &Supply<T>) -> Sure<T>
    where
        for<'mut, 'const> T: Clone,
    {
        when! {
            move => Sure(self.0),
            _ => Sure(self.0.clone()),
        }
    }
}
