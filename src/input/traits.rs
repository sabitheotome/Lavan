use super::adapters::cursor::Cursor;
use crate::parser::{
    adapters::slice::Slice,
    traits::{IterativeParser, Parse},
};

pub trait Stream: Iterator {
    type SaveState;

    fn savestate(&mut self) -> Self::SaveState;
    fn backtrack(&mut self, state: Self::SaveState);

    fn is_exhausted(&mut self) -> bool {
        let s = self.savestate();
        let a = self.next().is_none();
        self.backtrack(s);
        a
    }

    fn cursor(self) -> Cursor<Self>
    where
        Self: Sized,
    {
        Cursor::new(self)
    }

    fn from<T>(source: T) -> Self
    where
        Self: Sized,
        T: IntoStream<IntoStream = Self>,
    {
        source.into_scanner()
    }

    fn parse<T>(&mut self) -> T::Output
    where
        Self: Sized,
        T: Parse<Self>,
    {
        T::parse(self)
    }
}

pub trait StreamSlice: Stream {
    type Slice;
    type SliceOffset;

    fn slice_offset(&self) -> Self::SliceOffset;
    fn slice_since(&self, start: Self::SliceOffset) -> Self::Slice;
}

pub trait StreamSpan: Stream {
    type Span;
    type SpanOffset;

    fn span_offset(&self) -> Self::SpanOffset;
    fn span_since(&self, start: Self::SpanOffset) -> Self::Span;
}

pub trait StreamTrim: Stream {
    fn trim(&mut self);
}

pub trait IntoStream {
    type IntoStream: Stream;

    fn into_scanner(self) -> Self::IntoStream;

    fn evaluate<Par>(self, parser: Par) -> Par::Output
    where
        Self: Sized,
        Par: IterativeParser<Self::IntoStream>,
    {
        parser.evaluate(self)
    }
}

pub trait StrStream<'a>: StreamSlice<Item = char, Slice = &'a str> {}
impl<'a, T> StrStream<'a> for T where T: StreamSlice<Item = char, Slice = &'a str> {}
