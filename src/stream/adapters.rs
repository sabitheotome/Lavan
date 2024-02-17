use super::traits::{Stream, StreamSlice};

impl<'a, T> Stream for (&'a [T], usize)
where
    T: Clone,
{
    type Item = T;
    type Offset = usize;
    type Span = (usize, usize);
    type Peek<'b> = &'b T
    where
        Self: 'b;

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

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.0.get(self.offset()).cloned();
        self.skip();
        return char;
    }

    fn peek(&self) -> Option<Self::Peek<'_>> {
        self.0.get(self.offset())
    }

    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>> {
        self.0.get(self.offset() + offset)
    }

    fn span(&self, start: Self::Offset, end: Self::Offset) -> Self::Span {
        (start, end)
    }
}

impl<'a, T> StreamSlice<'a> for (&'a [T], usize)
where
    T: 'a + Clone,
{
    type Slice = &'a [T];

    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice {
        &self.0[start..end]
    }
}

impl<'b> Stream for (&'b str, usize) {
    type Item = char;
    type Offset = usize;
    type Span = (usize, usize);
    type Peek<'a> = char
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

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.0.chars().nth(self.offset());
        self.skip();
        return char;
    }

    fn peek(&self) -> Option<Self::Peek<'_>> {
        self.0.chars().nth(self.offset())
    }

    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>> {
        self.0.chars().nth(self.offset() + offset)
    }

    fn span(&self, start: Self::Offset, end: Self::Offset) -> Self::Span {
        (start, end)
    }
}

impl<'a> StreamSlice<'a> for (&'a str, usize) {
    type Slice = &'a str;

    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice {
        &self.0[start..end]
    }
}
