use std::fmt::Debug;

use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

/// TODO
///
/// This `struct` is created by the [`Parser::unwrapped`] method on [`Parser`].
/// See its documentation for more.
pub struct Unwrapped<Par> {
    parser: Par,
}

impl<Par> Unwrapped<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Output: Debug,
    {
        Self { parser }
    }
}

impl<Par> Parser for Unwrapped<Par>
where
    Par: Parser,
    Par::Output: Debug,
{
    type Input = Par::Input;
    type Output = ();

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        todo!()
    }
}
