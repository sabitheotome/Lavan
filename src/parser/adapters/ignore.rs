use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

pub struct Ignore<Par> {
    parser: Par,
}

impl<Par> Ignore<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Output: Ignorable,
    {
        Self { parser }
    }
}

impl<Par> Parser for Ignore<Par>
where
    Par: Parser,
    Par::Output: Ignorable,
{
    type Input = Par::Input;
    type Output = <Par::Output as Ignorable>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.parse_stream(input).ignore_response()
    }
}
