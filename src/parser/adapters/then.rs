use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::output::util::try_op;
use crate::parser::prelude::*;
use std::marker::PhantomData;

/// A parser for flat-mapping responses
///
/// This `struct` is created by the [`Parser::then`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Then<Par, Fun> {
    parser: Par,
    function: Fun,
}

impl<Par, Fun> Then<Par, Fun> {
    pub(crate) fn new(parser: Par, function: Fun) -> Self
    where
        Par: Operator,
        Par::Response: Apply<Fun>,
    {
        Self { parser, function }
    }
}

impl<Par, Fun> Operator for Then<Par, Fun>
where
    Par: Operator,
    Par::Response: Apply<Fun>,
{
    type Scanner = Par::Scanner;
    type Response = <Par::Response as Apply<Fun>>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser.parse_next(input).apply(&self.function)
    }
}
