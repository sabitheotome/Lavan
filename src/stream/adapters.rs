use super::traits::{IntoStream, Stream, StreamSlice, TokenSequence, TokenSlice};

impl<'a, T> TokenSequence for &'a [T]
where
    T: Clone,
{
    type Item = T;
    type Offset = usize;
    type Peek<'b> = &'b T
    where
        Self: 'b;

    fn nth(&self, offset: Self::Offset) -> Option<Self::Item> {
        self.peek_nth(offset).cloned()
    }

    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>> {
        self.get(offset)
    }
}

impl<'a, T> TokenSlice<'a> for &'a [T]
where
    T: 'a + Clone,
{
    type Slice = &'a [T];

    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice {
        &self[start..end]
    }
}

impl<'b> TokenSequence for &'b str {
    type Item = char;
    type Offset = usize;
    type Peek<'a> = char
    where
        Self: 'a;

    fn nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>> {
        self.peek_nth(offset)
    }

    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>> {
        self.chars().nth(offset)
    }
}

impl<'a> TokenSlice<'a> for &'a str {
    type Slice = &'a str;

    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice {
        &self[start..end]
    }
}

impl<T> Stream for (T, usize)
where
    T: TokenSequence<Offset = usize>,
{
    type Item = T::Item;
    type Offset = T::Offset;
    type Span = (usize, usize);
    type Peek<'a> = T::Peek<'a>
    where
        Self: 'a;

    fn offset(&self) -> Self::Offset {
        self.1
    }

    fn offset_mut(&mut self) -> &mut Self::Offset {
        &mut self.1
    }

    fn skip(&mut self) {
        self.advance(1);
    }

    fn advance(&mut self, offset: Self::Offset) {
        *self.offset_mut() += offset;
    }

    fn retract(&mut self) {
        self.go_back(1);
    }

    fn go_back(&mut self, offset: Self::Offset) {
        *self.offset_mut() -= offset;
    }

    fn nth(&mut self, offset: Self::Offset) -> Option<Self::Item> {
        *self.offset_mut() = offset + 1;
        self.0.nth(offset)
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.nth(self.offset())
    }

    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>> {
        self.0.peek_nth(offset)
    }

    fn peek(&self) -> Option<Self::Peek<'_>> {
        self.peek_nth(self.offset())
    }

    fn span(&self, start: Self::Offset, end: Self::Offset) -> Self::Span {
        (start, end)
    }
}

impl<'a, T> StreamSlice<'a> for (T, usize)
where
    T: TokenSlice<'a, Offset = usize>,
{
    type Slice = T::Slice;

    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice {
        self.0.slice(start, end)
    }
}

impl<T> IntoStream for T
where
    T: TokenSequence<Offset = usize>,
{
    type Stream = (T, usize);

    fn into_stream(self) -> Self::Stream {
        (self, 0)
    }
}
