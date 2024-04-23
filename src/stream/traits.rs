use crate::parser::traits::{Parse, Parser};

pub trait TokenSequence {
    type Token;
    type Peek<'a>
    where
        Self: 'a;
    type Offset;

    fn nth(&self, offset: Self::Offset) -> Option<Self::Token>;
    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>>;
}

pub trait TokenSlice<'a>: TokenSequence {
    type Slice: 'a;
    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice;
}

pub trait Stream {
    type Token;
    type Peek<'a>
    where
        Self: 'a;
    type Offset: Clone;

    fn offset(&self) -> Self::Offset;
    fn offset_mut(&mut self) -> &mut Self::Offset;

    fn skip(&mut self);
    fn advance(&mut self, offset: Self::Offset);
    fn retract(&mut self);
    fn go_back(&mut self, offset: Self::Offset);

    fn nth(&mut self, offset: Self::Offset) -> Option<Self::Token>;
    fn next(&mut self) -> Option<Self::Token>;
    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>>;
    fn peek(&self) -> Option<Self::Peek<'_>>;

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

impl<'a, T> Stream for &'a mut T
where
    T: Stream,
{
    type Token = T::Token;
    type Peek<'b>= T::Peek<'b>
    where
        Self: 'b;
    type Offset = T::Offset;

    fn offset(&self) -> Self::Offset {
        (**self).offset()
    }

    fn offset_mut(&mut self) -> &mut Self::Offset {
        (**self).offset_mut()
    }

    fn skip(&mut self) {
        (**self).skip()
    }

    fn advance(&mut self, offset: Self::Offset) {
        (**self).advance(offset)
    }

    fn retract(&mut self) {
        (**self).retract()
    }

    fn go_back(&mut self, offset: Self::Offset) {
        (**self).go_back(offset)
    }

    fn nth(&mut self, offset: Self::Offset) -> Option<Self::Token> {
        (**self).nth(offset)
    }

    fn next(&mut self) -> Option<Self::Token> {
        (**self).next()
    }

    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>> {
        (**self).peek_nth(offset)
    }

    fn peek(&self) -> Option<Self::Peek<'_>> {
        (**self).peek()
    }
}

pub trait StreamSlice<'a>: Stream {
    type Slice: 'a;
    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice;
}

pub trait IntoStream: TokenSequence {
    type Stream: Stream;
    fn into_stream(self) -> Self::Stream;
}

pub trait StrStream<'a>: StreamSlice<'a, Slice = &'a str, Token = char> {}
impl<'a, T> StrStream<'a> for T where T: StreamSlice<'a, Slice = &'a str, Token = char> {}

pub trait SliceStream<'a, E: 'a>: StreamSlice<'a, Slice = &'a [E], Token = E> {}
impl<'a, T, E: 'a> SliceStream<'a, E> for T where T: StreamSlice<'a, Slice = &'a [E], Token = E> {}
