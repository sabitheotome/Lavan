use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser for alternating two parsers through [`Switchable`]
///
/// This `struct` is created by the [`Parser::or`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Or<Par0, Par1> {
    parser0: Par0,
    parser1: Par1,
}

impl<Par0, Par1> Or<Par0, Par1> {
    pub(crate) fn new(parser0: Par0, parser1: Par1) -> Or<Par0, Par1>
    where
        Par0: Operator,
        Par1: Operator<Scanner = Par0::Scanner>,
        //Par0::Output: Switchable<<Par1::Output as Response>::WithVal<<Par0::Output as Response>::Value>>,
    {
        Or { parser0, parser1 }
    }
}

#[cfg(feature = "either")]
use either::Either;

#[cfg(feature = "either")]
impl<Par0, Par1> Or<Par0, Par1> {
    pub fn either(
        &self,
    ) -> Or<
        impl Operator<Response = val![Par0<Either<val![Par0], val![Par1]>>], Scanner = Par0::Scanner>
            + '_,
        impl Operator<Response = val![Par1<Either<val![Par0], val![Par1]>>], Scanner = Par1::Scanner>
            + '_,
    >
    where
        Par0: Operator,
        Par0::Response: ValueFunctor,
        Par1: Operator,
        Par1::Response: ValueFunctor,
        val![Par0<Either<val![Par0], val![Par1]>>]:
            Switch<val![Par1<Either<val![Par0], val![Par1]>>]>,
    {
        let parser0 = self
            .parser0
            .as_ref()
            .map(Either::<val![Par0], val![Par1]>::Left);
        let parser1 = self
            .parser1
            .as_ref()
            .map(Either::<val![Par0], val![Par1]>::Right);
        Or { parser0, parser1 }
    }
}

impl<Par0, Par1> Operator for Or<Par0, Par1>
where
    Par0: Operator,
    Par1: Operator<Scanner = Par0::Scanner>,
    Par0::Response: Switch<Par1::Response>,
{
    type Scanner = Par0::Scanner;
    type Response = <Par0::Response as Switch<Par1::Response>>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut save_state = input.savestate();
        self.parser0.parse_next(input).switch(|| {
            input.backtrack(save_state);
            self.parser1.parse_next(input)
        })
    }
}
