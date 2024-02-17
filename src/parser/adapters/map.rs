use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

pub type FnMap<Par, Val0, Val1> = Map<Par, fn(Val0) -> Val1>;

pub struct Map<Par, Fun> {
    parser: Par,
    function: Fun,
}

impl<Par, Fun> Map<Par, Fun> {
    pub(crate) fn new(parser: Par, function: Fun) -> Self
    where
        Par: Parser,
        Par::Output: Mappable<Fun>,
    {
        Map { parser, function }
    }
}

impl<Par, Fun> Parser for Map<Par, Fun>
where
    Par: Parser,
    Par::Output: Mappable<Fun>,
{
    type Input = Par::Input;
    type Output = <Par::Output as Mappable<Fun>>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.parse_stream(input).map_response(&self.function)
    }
}
