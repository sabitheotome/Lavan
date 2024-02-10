use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;
use std::marker::PhantomData;

pub struct TryMap<Par, Fun, Val> {
    parser: Par,
    function: Fun,
    _marker: PhantomData<Val>,
}

impl<Par, Fun, Val> TryMap<Par, Fun, Val> {
    pub(crate) fn new(parser: Par, function: Fun) -> TryMap<Par, Fun, Val>
    where
        Par: Parser,
        Par::Output: Pseudodata + Exceptional,
        Fun: Fn(val![Par]) -> val![Par<Val>],
    {
        TryMap {
            parser,
            function,
            _marker: PhantomData,
        }
    }
}

impl<Par, Fun, Val> Parser for TryMap<Par, Fun, Val>
where
    Par: Parser,
    Par::Output: Pseudodata + Exceptional,
    Fun: Fn(val![Par]) -> val![Par<Val>],
{
    type Input = Par::Input;
    type Output = val![Par<Val>];

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.parse_stream(input).flat_map(&self.function)
    }
}
