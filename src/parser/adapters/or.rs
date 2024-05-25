use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser for alternating two parsers through [`Switchable`]
///
/// This `struct` is created by the [`Parser::or`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct Or<Par0, Par1> {
    parser0: Par0,
    parser1: Par1,
}

impl<Par0, Par1> Or<Par0, Par1> {
    pub(crate) fn new(parser0: Par0, parser1: Par1) -> Or<Par0, Par1>
    where
        Par0: Parser,
        Par1: Parser<Input = Par0::Input>,
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
        impl Parser<Output = val![Par0<Either<val![Par0], val![Par1]>>], Input = Par0::Input> + '_,
        impl Parser<Output = val![Par1<Either<val![Par0], val![Par1]>>], Input = Par1::Input> + '_,
    >
    where
        Par0: Parser,
        Par0::Output: ValueFunctor,
        Par1: Parser,
        Par1::Output: ValueFunctor,
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

impl<Par0, Par1> Parser for Or<Par0, Par1>
where
    Par0: Parser,
    Par1: Parser<Input = Par0::Input>,
    Par0::Output: Switch<Par1::Output>,
{
    type Input = Par0::Input;
    type Output = <Par0::Output as Switch<Par1::Output>>::Output;

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        let mut save_state = input.savestate();
        self.parser0.next(input).switch(|| {
            input.backtrack(save_state);
            self.parser1.next(input)
        })
    }
}
