use crate::parser::prelude::*;
use crate::parser::util::assoc::err;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

// TODO
struct TryWith<Par0, Par1, Fun> {
    parser0: Par0,
    parser1: Par1,
    function: Fun,
}

impl<Par0, Par1, Fun> TryWith<Par0, Par1, Fun> {
    pub(crate) fn new(parser0: Par0, parser1: Par1, function: Fun) -> Self
    where
        Par0: Parser,
        Par1: Parser<Input = Par0::Input>,
        Par0::Output: Combinable<Par1::Output>,
    {
        Self {
            parser0,
            parser1,
            function,
        }
    }
}

impl<Par0, Par1, Fun> Parser for TryWith<Par0, Par1, Fun>
where
    Par0: Parser,
    Par1: Parser<Input = Par0::Input>,
    Par0::Output: Fallible,
    Par1::Output: Fallible,
{
    type Input = Par0::Input;
    type Output = ();

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        /*let value0 = self.parser0.parse_stream(input)?;
        let offset = input.offset();
        match self.parser1.parse_stream(input) {
            Ok(value1) => match (self.function)(value0, value1) {
                Ok(value) => Ok(value),
                Err(value) => {
                    input.set(offset);
                    Ok(value)
                }
            },
            Err(_) => {
                input.set(offset);
                Ok(value0)
            }
        }*/
        todo!()
    }
}
