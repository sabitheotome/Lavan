use crate::parser::prelude::internal::*;

/// A parser that yields the slice of the operation
///
/// This `struct` is created by the [`Parser::slice`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Slice<'a, Par> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) _marker: PhantomData<&'a ()>,
}

#[parser_fn]
fn slice<'a, par, Slc>(self: &Slice<'a, par>) -> <par::Output as Attach>::Output<Slc>
where
    INPUT: 'a + StreamSlice<Slice = Slc>,
    par::Output: Attach,
    Slc: 'a,
{
    let slice_builder = input.slice_offset();
    let result = parse![self.parser];
    let slice = input.slice_since(slice_builder);
    result.attach_to_response(move || slice)
}
