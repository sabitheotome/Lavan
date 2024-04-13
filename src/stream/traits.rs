use crate::parser::traits::{Parse, Parser};

pub trait Stream {
    type Item;
    type Peek<'a>
    where
        Self: 'a;
    type Offset;
    type Span;

    fn offset(&self) -> Self::Offset;
    fn offset_mut(&mut self) -> &mut Self::Offset;

    fn skip(&mut self);
    fn advance(&mut self, offset: Self::Offset);
    fn retract(&mut self);
    fn go_back(&mut self, offset: Self::Offset);

    fn nth(&mut self, offset: Self::Offset) -> Option<Self::Item>;
    fn next(&mut self) -> Option<Self::Item>;
    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>>;
    fn peek(&self) -> Option<Self::Peek<'_>>;

    fn span(&self, start: Self::Offset, end: Self::Offset) -> Self::Span;

    fn parse<T>(&mut self) -> T::Output
    where
        T: Parse<Input = Self>,
    {
        T::parse(self)
    }

    fn evaluate<Par>(&mut self, parser: Par) -> Par::Output
    where
        Self: Sized,
        Par: Parser<Input = Self>,
    {
        parser.parse_stream(self)
    }

    fn has_next(&self) -> bool {
        match self.peek() {
            Some(_) => false,
            None => true,
        }
    }
}

pub trait StreamSlice<'a>: Stream {
    type Slice: 'a;
    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice;
}
