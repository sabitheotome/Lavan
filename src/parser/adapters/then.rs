use crate::parser::prelude::*;
use crate::output::prelude::*;
use crate::output::util::try_op;
use crate::input::prelude::*;
use std::marker::PhantomData;

/// A parser for flat-mapping responses
///
/// This `struct` is created by the [`Parser::then`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct Then<Par, Fun> {
    parser: Par,
    function: Fun,
}

impl<Par, Fun> Then<Par, Fun> {
    pub(crate) fn new(parser: Par, function: Fun) -> Self
    where
        Par: Parser,
        Par::Output: Apply<Fun>,
    {
        Self { parser, function }
    }
}

impl<Par, Fun> Parser for Then<Par, Fun>
where
    Par: Parser,
    Par::Output: Apply<Fun>,
{
    type Input = Par::Input;
    type Output = <Par::Output as Apply<Fun>>::Output;

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.next(input).apply(&self.function)
    }
}
