use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::response::util::try_op;
use crate::stream::traits::{Stream, StreamSlice};
use std::marker::PhantomData;

/// A parser that yields the slice of the operation
///
/// This `struct` is created by the [`Parser::slice`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Clone, Copy, Debug)]
pub struct Slice<'a, Par> {
    pub(super) parser: Par,
    pub(super) _marker: PhantomData<&'a ()>,
}

impl<'a, Par> Slice<'a, Par> {
    pub fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Input: StreamSlice<'a>,
        Par::Output: Attachable,
    {
        Self {
            parser,
            _marker: PhantomData,
        }
    }
}

impl<'a, Par, Slc> Parser for Slice<'a, Par>
where
    Par: Parser,
    Par::Input: StreamSlice<'a, Slice = Slc>,
    Par::Output: Attachable,
{
    type Input = Par::Input;
    type Output = <Par::Output as Attachable>::Output<Slc>;

    #[inline]
    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        let start = input.offset();
        let result = self.parser.parse_stream(input);
        let end = input.offset();

        result.attach_to_response(|| input.slice(start, end))
    }
}
