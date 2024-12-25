use super::prelude::internal::*;

pub use functions::*;

pub mod functions {
    use super::{adapters::*, *};

    pub fn src<Par, I>(par: Par) -> Src<Par, I> {
        Src(par, PhantomData)
    }
    
    // TODO: Documentation
    pub fn nop<I: Stream>() -> Src<NOP, I> {
        src(NOP)
    }

    // TODO: Documentation
    pub fn eoi<I: Stream>() -> Src<EOI, I> {
        src(EOI)
    }

    // TODO: Documentation
    pub fn any<I: Stream>() -> Src<Any, I> {
        src(Any)
    }

    // TODO: Documentation
    pub fn any_if<Fun, I: Stream>(f: Fun) -> Src<AnyIf<Fun>, I>
    where
        Fun: Fn(&I::Item) -> bool,
    {
        src(AnyIf(f))
    }

    // TODO: Documentation
    pub fn any_then<Fun, I: Stream>(f: Fun) -> Src<AnyThen<Fun>, I>
    where
        AnyThen<Fun>: ParseOnce<I>,
    {
        src(AnyThen(f))
    }

    // TODO: Documentation
    pub fn any_eq<Rhs, I: Stream>(v: Rhs) -> Src<AnyEq<Rhs>, I>
    where
        I::Item: PartialEq<Rhs>,
    {
        src(AnyEq(v))
    }

    // TODO: Documentation
    pub fn any_ne<Rhs, I: Stream>(v: Rhs) -> Src<AnyNe<Rhs>, I>
    where
        I::Item: PartialEq<Rhs>,
    {
        src(AnyEq(v))
    }

    // TODO: Documentation
    pub fn take<'a, I, Ref>(size: usize) -> Src<Take<'a>, I>
    where
        I: StreamSlice<Slice = &'a Ref>,
        Ref: 'a + std::ops::Index<std::ops::RangeTo<usize>> + ?Sized,
    {
        src(Take(size, PhantomData))
    }

    // TODO: Documentation
    pub fn func<'a, Fun, I: Stream, Out>(f: Fun) -> Src<Func<Fun, Out>, I>
    where
        Out: Response,
        Fun: FnOnce(&mut I) -> Out,
    {
        src(Func(f, PhantomData))
    }

    // TODO: Documentation
    pub fn mk<T, I: Stream>() -> Src<Mk<T>, I>
    where
        T: FromParse<I>,
    {
        src(Mk(PhantomData))
    }

    // TODO: Documentation
    pub fn mkdef<T, I: Stream>() -> Src<MkDefault<T>, I>
    where
        T: Default,
    {
        src(MkDefault(PhantomData))
    }

    // TODO: Documentation
    pub fn supply<T, I: Stream>(v: T) -> Src<Supply<T>, I> {
        src(Supply(v))
    }

    // TODO: Documentation
    pub fn recursive<'a, Out, Par, Fun, I>(f: Fun) -> Src<Recursive<Par>, I>
    where
        I: Stream,
        Par: 'a + Parse<I, Output = Out>,
        Fun: Fn(Weak<dyn 'a + Parse<I, Output = Out>>) -> Par,
    {
        src(Recursive {
            parser: std::rc::Rc::new_cyclic(|weak| {
                f(Weak {
                    parser: weak.clone()
                        as std::rc::Weak<dyn Parse<I, Output = Par::Output>>,
                })
            }),
        })
    }
}

pub mod adapters {
    use super::*;

    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
    /// See its documentation for more.    
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug)]
    pub struct Src<Par, I>(pub(crate) Par, pub(crate) std::marker::PhantomData<I>);

    impl<Par, I> Src<Par, I> {
        pub fn inner(self) -> Par {
            self.0
        }
    }

    #[parser_fn]
    fn infer<par>(self: &Src<par, INPUT>) -> par::Output
    where
        par: IterativeParser<INPUT>,
    {
        parse![self.0]
    }

    impl<Par: Clone, I> Clone for Src<Par, I> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }

    impl<Par: Copy, I> Copy for Src<Par, I> {}

    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
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

    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct AnyThen<Fun>(pub(crate) Fun);

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

    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct Func<Fun, Out>(pub(crate) Fun, pub(crate) PhantomData<Out>);

    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy)]
    pub struct Mk<T>(pub(crate) PhantomData<T>);

    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct MkDefault<T>(pub(crate) PhantomData<T>);

    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone, Copy)]
    pub struct Supply<T>(pub(crate) T);
        
    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
    /// See its documentation for more
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone)]
    pub struct Recursive<Par: ?Sized> {
        pub(crate) parser: std::rc::Rc<Par>,
    }

    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[derive(Debug, Clone)]
    pub struct Weak<Par: ?Sized> {
        pub(crate) parser: std::rc::Weak<Par>,
    }
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
        for<'once> Fun: FnOnce(&INPUT::Item) -> bool,
        for<'mut> Fun: FnMut(&INPUT::Item) -> bool,
        for<'const> Fun: Fn(&INPUT::Item) -> bool,
    {
        let mut peekable = input.peekable();
        let peek = peekable.peek()?;
        ((self.0)(peek)).then_some(peekable.next()?)
    }

    #[parser_fn]
    fn any_then<Fun, Out>(self: &AnyThen<Fun>) -> Out
    where
        Out: Response,
        for<'a> Option<INPUT::Item>: Apply<&'a Fun, Output = Out>,
    {
        any().then(&self.0).parse_once(input)
    }

    #[parser_fn]
    fn any_eq<Rhs>(self: &AnyEq<Rhs>) -> Option<INPUT::Item>
    where
        INPUT::Item: PartialEq<Rhs>,
    {
        parse![any_if(|v| *v == self.0)]
    }

    #[parser_fn]
    fn any_ne<Rhs>(self: &AnyNe<Rhs>) -> Option<INPUT::Item>
    where
        INPUT::Item: PartialEq<Rhs>,
    {
        parse![any_if(|v| *v != self.0)]
    }

    #[parser_fn]
    fn take<'a, Ref>(self: &Take<'a>) -> Option<&'a Ref>
    where
        INPUT: Stream + StreamSlice<Slice = &'a Ref>,
        Ref: 'a + ?Sized,
    {
        any().del().repeat_min(self.0).slice().parse_once(input)
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
        T: FromParse<INPUT>,
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

    #[parser_fn]
    fn recursive<Par>(self: &Recursive<Par>) -> Par::Output
    where
        Par: ?Sized + Parse<INPUT>,
    {
        self.parser.parse(input)
    }

    #[parser_fn]
    fn weak<Par>(self: &Weak<Par>) -> Par::Output
    where
        Par: ?Sized + Parse<INPUT>,
    {
        self.parser.upgrade().unwrap().parse(input)
    }
}
