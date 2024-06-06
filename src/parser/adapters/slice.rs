use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::output::util::try_op;
use crate::parser::prelude::*;
use std::marker::PhantomData;
use std::ops::RangeTo;

/// A parser that yields the slice of the operation
///
/// This `struct` is created by the [`Parser::slice`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Slice<'a, Par> {
    pub(super) parser: Par,
    pub(super) _marker: PhantomData<&'a ()>,
}

impl<'a, Par> Slice<'a, Par> {
    pub fn new(parser: Par) -> Self
    where
        Par: Operator,
        Par::Scanner: ScannerSlice,
        Par::Response: Attachable,
    {
        Self {
            parser,
            _marker: PhantomData,
        }
    }
}

impl<'a, Par, Slc> Operator for Slice<'a, Par>
where
    Par: Operator,
    Par::Scanner: 'a + ScannerSlice<Slice = Slc>,
    Par::Response: Attachable,
    Slc: 'a,
{
    type Scanner = Par::Scanner;
    type Response = <Par::Response as Attachable>::Output<Slc>;

    #[inline]
    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let slice_builder = input.slice_offset();
        let result = self.parser.parse_next(input);
        let slice = input.slice_since(slice_builder);
        result.attach_to_response(move || slice)
    }
}
