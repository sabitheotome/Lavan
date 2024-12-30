use crate::parser::{
    prelude::internal::*,
    sources::{adapters::*, *},
};

use adapters::*;
use mode::*;

/// A parser for repeatition and collection
///
/// This `struct` is created by the [`Parser::repeat`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Repeater<Par, Sep = (), Mod = UntilErr, Col = NOP> {
    parser: Par,
    separator: Sep,
    mode: Mod,
    collector: Col,
}

pub mod adapters {
    pub use super::Repeater;
    use super::*;

    // TODO: Documentation
    pub type Repeat<Par, Sep = (), Col = NOP> = Repeater<Par, Sep, UntilErr, Col>;

    // TODO: Documentation
    pub type RepeatEOI<Par, Sep = (), Col = NOP> = Repeater<Par, Sep, UntilEOI, Col>;

    // TODO: Documentation
    pub type RepeatFuse<Par, Sep = (), Col = NOP> = Repeater<Par, Sep, UntilFuse, Col>;

    // TODO: Documentation
    pub type RepeatMin<Par, Sep = (), Col = NOP> = Repeater<Par, Sep, Minimum, Col>;

    // TODO: Documentation
    pub type RepeatMinEOI<Par, Sep = (), Col = NOP> = Repeater<Par, Sep, MinimumEOI, Col>;

    // TODO: Documentation
    pub type RepeatMax<Par, Sep = (), Col = NOP> = Repeater<Par, Sep, Maximum, Col>;

    // TODO: Documentation
    pub type RepeatExact<Par, Sep = (), Col = NOP> = Repeater<Par, Sep, Exact, Col>;
}

pub mod mode {
    use std::marker::PhantomData;

    #[derive(Clone, Copy, Debug)]
    pub struct By<Par>(pub(crate) Par);

    #[non_exhaustive]
    #[derive(Clone, Copy, Debug)]
    pub struct UntilErr;

    #[non_exhaustive]
    #[derive(Clone, Copy, Debug)]
    pub struct UntilEOI;

    #[non_exhaustive]
    #[derive(Clone, Copy, Debug)]
    pub struct UntilFuse;

    #[derive(Clone, Copy, Debug)]
    pub struct Minimum(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct MinimumEOI(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct Maximum(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct Exact(pub(crate) usize);
}

mod macros {
    macro_rules! safe_parse_flow {
        (@ $parser:expr) => {
            parser![$parser]
                .unchecked_auto_bt()
                .parse_once(input!())
                .control_flow()
        };
        ($parser:expr) => {
            parser![$parser]
                .auto_bt()
                .parse_once(input!())
                .control_flow()
        };
    }

    pub(super) use safe_parse_flow;

    macro_rules! try_parse {
        (use $t:tt => $parser:expr) => {
            tryexpr!(parse![use $t => $parser])
        };
        ($parser:expr) => {
            tryexpr!(parse![$parser])
        };
        ($parser:expr => ?fallible else $default:expr) => {
            match safe_parse_flow![@ $parser] {
                ControlFlow::Continue(var) => var,
                ControlFlow::Break(_err) => return tryok!($default),
            }
        };
        ($parser:expr => else $default:expr) => {
            match safe_parse_flow![$parser] {
                ControlFlow::Continue(var) => var,
                ControlFlow::Break(_err) => return tryok!($default),
            }
        };
        ($parser:expr => if #, $default:expr) => {
            try_parse![$parser => if input!().next().is_none(), $default]
        };
        ($parser:expr => if $cond:expr, $default:expr) => {
            match parse![$parser].branch() {
                ControlFlow::Continue(var) => var,
                ControlFlow::Break(res) => {
                    if $cond {
                        return tryok!($default);
                    } else {
                        return tryres!(res);
                    }
                }
            }
        };
    }

    pub(super) use try_parse;
}

mod impls {
    use super::{macros::*, *};

    #[parser_fn(mut in move)]
    fn repeat<par, col: DenyMutInMove>(mut self: &Repeat<par, (), col>) -> col::Output
    where
        par::Output: Fallible,
        val![col]: Extend<val![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        loop {
            collector.extend([try_parse![self.parser => else collector]]);
        }
    }

    #[parser_fn(mut in move)]
    fn repeat_sep<par, sep, col: DenyMutInMove>(
        mut self: &Repeat<par, By<sep>, col>,
    ) -> val![col in par]
    where
        sep::Output: Fallible,
        val![col]: Extend<val![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        // first iteration
        collector.extend([try_parse![self.parser => ?fallible else collector]]);
        loop {
            // return if separator was not found
            try_parse![self.separator.0 => else collector];
            // expects primary since a separator was found
            collector.extend([try_parse![self.parser]]);
        }
    }

    #[parser_fn(mut in move)]
    fn repeat_eoi<par, col: DenyMutInMove>(mut self: &RepeatEOI<par, (), col>) -> val![col in par]
    where
        val![col]: Extend<val![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        loop {
            if input.is_exhausted() {
                return tryok!(collector);
            }
            collector.extend([try_parse![self.parser]]);
        }
    }

    #[parser_fn(mut in move)]
    fn repeat_eoi_sep<par, sep, col: DenyMutInMove>(
        mut self: &RepeatEOI<par, By<sep>, col>,
    ) -> val![col in par]
    where
        val![col]: Extend<val![par]>,
        lifterr![sep]: IntoErr<err![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        // first iteration
        collector.extend([try_parse![self.parser => if #, collector]]);

        loop {
            if input.is_exhausted() {
                return tryok!(collector);
            }
            // break if separator was not found
            try_parse![self.separator.0];
            // expects parser since a separator was found
            collector.extend([try_parse![self.parser]]);
        }
    }

    #[parser_fn(mut in move)]
    fn repeat_fuse<par, col: DenyMutInMove>(mut self: &RepeatFuse<par, (), col>) -> val![col in par]
    where
        par::Output: Fallible,
        val![col]: Extend<val![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        loop {
            collector.extend([try_parse![self.parser => if #, collector]]);
        }
    }

    #[parser_fn(mut in move)]
    fn repeat_fuse_sep<par, sep, col: DenyMutInMove>(
        mut self: &RepeatFuse<par, By<sep>, col>,
    ) -> val![col in par]
    where
        sep::Output: Fallible,
        val![col]: Extend<val![par]>,
        lifterr![sep]: IntoErr<err![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        // first iteration
        collector.extend([try_parse![self.parser => if #, collector]]);

        loop {
            // break if separator was not found
            try_parse![self.separator.0 => if #, collector];
            // expects parser since a separator was found
            collector.extend([try_parse![self.parser]]);
        }
    }

    #[parser_fn(mut in move)]
    fn repeat_max<par, col: DenyMutInMove>(mut self: &RepeatMax<par, (), col>) -> col::Output
    where
        val![col]: Extend<val![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        for _ in 0..self.mode.0 {
            try_parse![self.parser  => ?fallible else collector];
        }

        return tryok![collector];
    }

    #[parser_fn(mut in move)]
    fn repeat_max_sep<par, sep, col: DenyMutInMove>(
        mut self: &RepeatMax<par, By<sep>, col>,
    ) -> val![col in par]
    where
        val![col]: Extend<val![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        // first iteration
        collector.extend([try_parse![self.parser => ?fallible else collector]]);
        for _ in 1..self.mode.0 {
            // return if separator was not found
            try_parse![self.separator.0 => ?fallible else collector];
            // expects primary since a separator was found
            collector.extend([try_parse![self.parser]]);
        }

        return tryok![collector];
    }

    #[parser_fn(mut in move)]
    fn repeat_exact<par, col: DenyMutInMove>(
        mut self: &RepeatExact<par, (), col>,
    ) -> val![col in par]
    where
        val![col]: Extend<val![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];
        for _ in 0..self.mode.0 {
            collector.extend([try_parse![self.parser]]);
        }

        return tryok![collector];
    }

    #[parser_fn(mut in move)]
    fn repeat_exact_sep<par, sep, col: DenyMutInMove>(
        mut self: &RepeatExact<par, By<sep>, col>,
    ) -> val![col in par]
    where
        val![col]: Extend<val![par]>,
        lifterr![sep]: IntoErr<err![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        // first iteration
        collector.extend([try_parse![self.parser]]);
        for _ in 1..self.mode.0 {
            // return if separator was not found
            try_parse![self.separator.0];
            // expects primary since a separator was found
            collector.extend([try_parse![self.parser]]);
        }

        return tryok![collector];
    }

    #[parser_fn(mut in move)]
    fn repeat_min<par, col: DenyMutInMove>(mut self: &RepeatMin<par, (), col>) -> val![col in par]
    where
        par::Output: Fallible,
        val![col]: Extend<val![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let collection = tryexpr!(parser![self.parser]
            .repeat_exact(self.mode.0)
            .collector(parser![use [not(mut in move)] => self.collector])
            .parse_once(input));
        tryok!(parser![self.parser]
            .repeat()
            .collect_into(collection)
            .parse_once(input)
            .value())
    }

    #[parser_fn(mut in move)]
    fn repeat_min_sep<par, sep, col: DenyMutInMove>(
        mut self: &RepeatMin<par, By<sep>, col>,
    ) -> val![col in par]
    where
        sep::Output: Fallible,
        val![col]: Extend<val![par]>,
        lifterr![sep]: IntoErr<err![par]>,
        lifterr![col]: IntoErr<err![par]>,
        val![col in par]: Response<Value = val![col], Error = err![par]>,
    {
        let mut collector = try_parse![use [not(mut in move)] => self.collector];

        // first iteration
        collector.extend([try_parse![self.parser]]);
        for _ in 1..self.mode.0 {
            try_parse![self.separator.0];
            collector.extend([try_parse![self.parser]]);
        }
        loop {
            try_parse![self.separator.0 => else collector];
            collector.extend([try_parse![self.parser]]);
        }

        return tryok![collector];
    }
}

impl<Par> Repeater<Par> {
    #[inline(always)]
    pub(crate) fn new(parser: Par) -> Self {
        Self {
            parser,
            separator: (),
            mode: UntilErr,
            collector: NOP,
        }
    }
}

impl<Par, Sep, Mod> Repeater<Par, Sep, Mod> {
    #[inline]
    pub fn collector<Col>(self, c: Col) -> Repeater<Par, Sep, Mod, Col> {
        Repeater {
            parser: self.parser,
            separator: self.separator,
            mode: self.mode,
            collector: c,
        }
    }

    #[inline]
    pub fn collect_into<Col>(self, c: Col) -> Repeater<Par, Sep, Mod, Supply<Col>> {
        self.collector(Supply(c))
    }

    #[inline]
    pub fn collect_def<Col: Default>(self) -> Repeater<Par, Sep, Mod, MkDefault<Col>> {
        self.collector(MkDefault(PhantomData))
    }

    #[inline]
    pub fn collect_mk<Col: Default>(self) -> Repeater<Par, Sep, Mod, Mk<Col>> {
        self.collector(Mk(PhantomData))
    }

    #[inline]
    pub fn collect<Col: util::Reserved>(self) -> Repeater<Par, Sep, Mod, util::MkReserved<Col>>
    where
        Mod: util::SizeHint,
    {
        let size = self.mode.size_hint();
        self.collector(util::MkReserved {
            size,
            phantom: PhantomData,
        })
    }

    #[inline]
    pub fn to_vec<T>(self) -> Repeater<Par, Sep, Mod, util::MkReserved<Vec<T>>>
    where
        Mod: util::SizeHint,
    {
        self.collect::<Vec<T>>()
    }
}

impl<Par, Sep, Mod, Col> Repeater<Par, Sep, Mod, util::MkReserved<Col>> {
    #[inline]
    pub fn flatten(self) -> Repeater<Par, Sep, Mod, util::MkReserved<util::FlatCollect<Col>>> {
        Repeater {
            parser: self.parser,
            separator: self.separator,
            mode: self.mode,
            collector: util::MkReserved {
                size: self.collector.size,
                phantom: PhantomData,
            },
        }
    }
}

impl<Par, Mod, Col> Repeater<Par, (), Mod, Col> {
    #[inline]
    pub fn separate_by<Sep>(self, s: Sep) -> Repeater<Par, By<Sep>, Mod, Col> {
        self.separator(By(s))
    }
}

impl<Par, Sep, Mod, Col> Repeater<Par, Sep, Mod, Col> {
    #[inline]
    fn separator<NewSep>(self, s: NewSep) -> Repeater<Par, NewSep, Mod, Col> {
        Repeater {
            parser: self.parser,
            separator: s,
            mode: self.mode,
            collector: self.collector,
        }
    }
}

impl<Par, Sep, Col> Repeater<Par, Sep, UntilErr, Col> {
    #[inline]
    pub fn until_eoi(self) -> RepeatEOI<Par, Sep, Col> {
        Repeater {
            parser: self.parser,
            separator: self.separator,
            mode: UntilEOI,
            collector: self.collector,
        }
    }

    #[inline]
    pub fn until_fuse(self) -> RepeatFuse<Par, Sep, Col> {
        Repeater {
            parser: self.parser,
            separator: self.separator,
            mode: UntilFuse,
            collector: self.collector,
        }
    }

    #[inline]
    pub fn max(self, m: usize) -> RepeatMax<Par, Sep, Col> {
        Repeater {
            parser: self.parser,
            separator: self.separator,
            mode: Maximum(m),
            collector: self.collector,
        }
    }

    #[inline]
    pub fn min(self, m: usize) -> RepeatMin<Par, Sep, Col> {
        Repeater {
            parser: self.parser,
            separator: self.separator,
            mode: Minimum(m),
            collector: self.collector,
        }
    }

    #[inline]
    pub fn exact(self, e: usize) -> RepeatExact<Par, Sep, Col> {
        Repeater {
            parser: self.parser,
            separator: self.separator,
            mode: Exact(e),
            collector: self.collector,
        }
    }
}

impl<Par, Sep, Col> RepeatMin<Par, Sep, Col> {
    #[inline]
    pub fn until_eoi(self) -> RepeatMinEOI<Par, Sep, Col> {
        Repeater {
            parser: self.parser,
            separator: self.separator,
            mode: MinimumEOI(self.mode.0),
            collector: self.collector,
        }
    }
}

pub mod util {
    use super::*;

    pub struct FlatCollect<Col> {
        collection: Col,
    }

    impl<Col> FlatCollect<Col> {
        pub fn inner(self) -> Col {
            self.collection
        }
    }

    impl<Res, Col> Extend<Res> for FlatCollect<Col>
    where
        Res: IntoIterator,
        Col: Extend<Res::Item>,
    {
        fn extend<T: IntoIterator<Item = Res>>(&mut self, iter: T) {
            self.collection
                .extend(iter.into_iter().map(|r| r.into_iter()).flatten())
        }
    }

    pub trait Reserved {
        fn reserved(size: usize) -> Self;
    }

    impl<T> Reserved for Vec<T> {
        fn reserved(size: usize) -> Self {
            Vec::with_capacity(size)
        }
    }

    impl<T> Reserved for FlatCollect<T>
    where
        T: Reserved,
    {
        fn reserved(size: usize) -> Self {
            Self {
                collection: T::reserved(size),
            }
        }
    }

    pub struct MkReserved<Col> {
        pub(super) size: usize,
        pub(super) phantom: PhantomData<Col>,
    }

    #[parser_fn]
    fn reserved<Col>(self: &MkReserved<Col>) -> Sure<Col>
    where
        Col: Reserved,
    {
        Sure(Col::reserved(self.size))
    }

    pub trait SizeHint {
        fn size_hint(&self) -> usize;
    }

    impl SizeHint for UntilErr {
        fn size_hint(&self) -> usize {
            0
        }
    }

    impl SizeHint for UntilEOI {
        fn size_hint(&self) -> usize {
            0
        }
    }

    impl SizeHint for Exact {
        fn size_hint(&self) -> usize {
            self.0
        }
    }

    impl SizeHint for Maximum {
        fn size_hint(&self) -> usize {
            self.0
        }
    }

    impl SizeHint for Minimum {
        fn size_hint(&self) -> usize {
            self.0
        }
    }

    impl SizeHint for MinimumEOI {
        fn size_hint(&self) -> usize {
            self.0
        }
    }
}
