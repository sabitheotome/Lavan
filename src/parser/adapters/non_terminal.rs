use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

pub struct NonTerminal<Par> {
    parser: Par,
}

impl<Par> NonTerminal<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Output: Recoverable,
    {
        Self { parser }
    }
}

impl<Par> Parser for NonTerminal<Par>
where
    Par: Parser,
    Par::Output: Recoverable,
{
    type Input = Par::Input;
    type Output = Par::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        let offset = input.offset();
        self.parser.parse_stream(input).recover_response(
            |input| {
                *input.offset_mut() = offset;
            },
            input,
        )
    }
}
