use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

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
        Par0::Output: Switchable<Par1::Output>,
    {
        Or { parser0, parser1 }
    }
}

#[cfg(feature = "either")]
use either::Either;

#[cfg(feature = "either")]
impl<Par0, Par1> Or<Par0, Par1> {
    pub fn either<'a, Str>(&'a self) -> Or<impl Parser + 'a, impl Parser + 'a>
    where
        Par0: Parser,
        Par0::Output: 'a + ValueFunctor,
        Par1: Parser,
        Par1::Output: 'a + ValueFunctor,
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
    Par0::Output: Switchable<Par1::Output>,
{
    type Input = Par0::Input;
    type Output = <Par0::Output as Switchable<Par1::Output>>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        let offset = input.offset();
        self.parser0.parse_stream(input).disjoin_response(
            |str| self.parser1.parse_stream(str),
            |str| *str.offset_mut() = offset,
            input,
        )
    }
}
