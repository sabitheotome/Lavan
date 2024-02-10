use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

pub struct AsRef<'a, Par> {
    parser: &'a Par,
}

impl<'a, Par> AsRef<'a, Par> {
    pub(crate) fn new(parser: &'a Par) -> Self
    where
        Par: Parser,
    {
        Self { parser }
    }
}

impl<'a, Par> Parser for AsRef<'a, Par>
where
    Par: Parser,
{
    type Input = Par::Input;
    type Output = Par::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.parse_stream(input)
    }
}
