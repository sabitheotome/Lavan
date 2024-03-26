use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::response::util::try_op;
use crate::stream::traits::Stream;
use std::marker::PhantomData;

/// A parser for flat-mapping [`Fallible`] responses
///
/// This `struct` is created by the [`Parser::try_map`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct TryMap<Par, Fun, Val> {
    parser: Par,
    function: Fun,
    _marker: PhantomData<Val>,
}

impl<Par, Fun, Val> TryMap<Par, Fun, Val> {
    pub(crate) fn new(parser: Par, function: Fun) -> Self
    where
        Par: Parser,
        Par::Output: ValueFunctor + Fallible,
        Fun: Fn(val![Par]) -> val![Par<Val>],
    {
        Self {
            parser,
            function,
            _marker: PhantomData,
        }
    }
}

impl<Par, Fun, Val> Parser for TryMap<Par, Fun, Val>
where
    Par: Parser,
    Par::Output: ValueFunctor + Fallible,
    Fun: Fn(val![Par]) -> val![Par<Val>],
{
    type Input = Par::Input;
    type Output = val![Par<Val>];

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.parse_stream(input).flat_map(&self.function)
    }
}
