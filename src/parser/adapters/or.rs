use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// A parser for alternating two parsers through [`Switchable`]
///
/// This `struct` is created by the [`Parser::or`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Or<Par0, Par1> {
    pub(in crate::parser) parser0: Par0,
    pub(in crate::parser) parser1: Par1,
}

#[parser_fn]
fn or<par0, par1>(self: &Or<par0, par1>) -> <par0::Output as Switch<par1::Output>>::Output
where
    par0::Output: Switch<par1::Output>,
{
    let mut save_state = input.savestate();
    parse![self.parser0].switch(|| {
        input.backtrack(save_state);
        parse![self.parser1]
    })
}

#[cfg(feature = "either")]
use either::Either;

#[cfg(feature = "either")]
impl<Par0, Par1> Or<Par0, Par1> {
    pub fn either<Input: Stream>(
        self,
    ) -> Or<
        impl IterativeParser<Input, Output = val![Par0<Either<val![Par0], val![Par1]>>]>,
        impl IterativeParser<Input, Output = val![Par1<Either<val![Par0], val![Par1]>>]>,
    >
    where
        Par0: IterativeParser<Input>,
        Par0::Output: ValueResponse,
        Par1: IterativeParser<Input>,
        Par1::Output: ValueResponse,
        val![Par0<Either<val![Par0], val![Par1]>>]:
            Switch<val![Par1<Either<val![Par0], val![Par1]>>]>,
    {
        let parser0 = self.parser0.sel(Either::<val![Par0], val![Par1]>::Left);
        let parser1 = self.parser1.sel(Either::<val![Par0], val![Par1]>::Right);
        Or { parser0, parser1 }
    }
}
