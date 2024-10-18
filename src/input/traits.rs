use super::adapters::cursor::Cursor;
use crate::parser::{
    adapters::slice::Slice,
    traits::{Parse, Parser},
};

pub trait Scanner: Iterator {
    type SaveState;

    fn savestate(&mut self) -> Self::SaveState;

    fn backtrack(&mut self, state: Self::SaveState);

    fn cursor(self) -> Cursor<Self>
    where
        Self: Sized,
    {
        Cursor::new(self)
    }

    fn from<T>(source: T) -> Self
    where
        Self: Sized,
        T: IntoScanner<IntoScanner = Self>,
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

pub trait ScannerSlice: Scanner {
    type Slice;
    type SliceOffset;

    fn slice_offset(&self) -> Self::SliceOffset;
    fn slice_since(&self, start: Self::SliceOffset) -> Self::Slice;
}

pub trait ScannerSpan: Scanner {
    type Span;
    type SpanOffset;

    fn span_offset(&self) -> Self::SpanOffset;
    fn span_since(&self, start: Self::SpanOffset) -> Self::Span;
}

pub trait ScannerTrim: Scanner {
    fn trim(&mut self);
}

pub trait IntoScanner {
    type IntoScanner: Scanner;

    fn into_scanner(self) -> Self::IntoScanner;

    fn evaluate<Par>(self, parser: Par) -> Par::Output
    where
        Self: Sized,
        Par: Parser<Self::IntoScanner>,
    {
        parser.evaluate(self)
    }
}

pub trait StrStream<'a>: ScannerSlice<Item = char, Slice = &'a str> {}
impl<'a, T> StrStream<'a> for T where T: ScannerSlice<Item = char, Slice = &'a str> {}
