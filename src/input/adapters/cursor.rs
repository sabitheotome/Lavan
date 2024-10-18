use crate::prelude::{IntoScanner, Scanner, ScannerSlice, ScannerSpan};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Cursor<S> {
    pub scanner: S,
    pub count: usize,
}

impl<S> Cursor<S> {
    pub fn new(scanner: S) -> Self {
        Self { scanner, count: 0 }
    }
}

impl<S> Iterator for Cursor<S>
where
    S: Iterator,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        self.scanner.next()
    }
}

impl<S> Scanner for Cursor<S>
where
    S: Scanner,
{
    type SaveState = S::SaveState;

    fn savestate(&mut self) -> Self::SaveState {
        self.scanner.savestate()
    }

    fn backtrack(&mut self, state: Self::SaveState) {
        self.scanner.backtrack(state)
    }
}

impl<S> ScannerSlice for Cursor<S>
where
    S: ScannerSlice,
{
    type Slice = S::Slice;
    type SliceOffset = S::SliceOffset;

    fn slice_offset(&self) -> Self::SliceOffset {
        self.scanner.slice_offset()
    }

    fn slice_since(&self, start: Self::SliceOffset) -> Self::Slice {
        self.scanner.slice_since(start)
    }
}

impl<S> ScannerSpan for Cursor<S>
where
    S: Scanner,
{
    type Span = (usize, usize);
    type SpanOffset = usize;

    fn span_offset(&self) -> Self::SpanOffset {
        self.count
    }

    fn span_since(&self, start: Self::SpanOffset) -> Self::Span {
        (start, self.count)
    }
}
