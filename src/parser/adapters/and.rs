use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;
use crate::parser::util::assoc::err;

/// A parser for combining two parsers through [`Combinable`].
///
/// This `struct` is created by the [`Parser::and`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct And<Par0, Par1> {
    parser0: Par0,
    parser1: Par1,
}

impl<Par0, Par1> And<Par0, Par1> {
    pub(crate) fn new(parser0: Par0, parser1: Par1) -> And<Par0, Par1>
    where
        Par0: Operator,
        Par1: Operator<Scanner = Par0::Scanner>,
        Par0::Response: Combine<Par1::Response>,
    {
        And { parser0, parser1 }
    }

    #[cfg(feature = "either")]
    pub fn either_err<Input>(self) -> And<impl Operator, impl Operator>
    where
        Input: Scanner,
        Par0: Parser<Input>,
        Par1: Parser<Input>,
        Par0::Output: ErrorFunctor,
        Par1::Output: ErrorFunctor,
    {
        use either::Either;

        And {
            parser0: self
                .parser0
                .map_err(Either::<err![Par0::Output], err![Par1::Output]>::Left),
            parser1: self
                .parser1
                .map_err(Either::<err![Par0::Output], err![Par1::Output]>::Right),
        }
    }
}

impl<Par0, Par1> Operator for And<Par0, Par1>
where
    Par0: Operator,
    Par1: Operator<Scanner = Par0::Scanner>,
    Par0::Response: Combine<Par1::Response>,
{
    type Scanner = Par0::Scanner;
    type Response = <Par0::Response as Combine<Par1::Response>>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser0
            .parse_next(input)
            .combine(|| self.parser1.parse_next(input))
    }
}
