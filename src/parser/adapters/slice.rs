use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::output::prelude::*;
use crate::output::util::try_op;
use std::marker::PhantomData;
use std::ops::RangeTo;

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
        Par::Input: ScannerSlice,
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
    Par::Input: 'a + ScannerSlice<Slice = Slc>,
    Par::Output: Attachable,
    Slc: 'a,
{
    type Input = Par::Input;
    type Output = <Par::Output as Attachable>::Output<Slc>;

    #[inline]
    fn next(&self, input: &mut Self::Input) -> Self::Output {
        let slice_builder = input.slice_offset();
        let result = self.parser.next(input);
        let slice = input.slice_since(slice_builder);
        result.attach_to_response(move || slice)
    }
}
