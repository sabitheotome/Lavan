use crate::parser::prelude::*;
use crate::parser::util::assoc::err;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

pub struct And<Par0, Par1> {
    parser0: Par0,
    parser1: Par1,
}

impl<Par0, Par1> And<Par0, Par1> {
    pub(crate) fn new(parser0: Par0, parser1: Par1) -> And<Par0, Par1>
    where
        Par0: Parser,
        Par1: Parser<Input = Par0::Input>,
        Par0::Output: Combinable<Par1::Output>,
    {
        And { parser0, parser1 }
    }
}

impl<Par0, Par1> And<Par0, Par1> {
    #[cfg(feature = "either")]
    pub fn either_err<'a>(&'a self) -> And<impl Parser + 'a, impl Parser + 'a>
    where
        Par0: Parser,
        Par0::Output: 'a + Exceptional,
        Par1: Parser,
        Par1::Output: 'a + Exceptional,
    {
        use either::Either;

        And {
            parser0: self.parser0.map_err(Either::<err![Par0], err![Par1]>::Left),
            parser1: self
                .parser1
                .map_err(Either::<err![Par0], err![Par1]>::Right),
        }
    }
}

impl<Par0, Par1> Parser for And<Par0, Par1>
where
    Par0: Parser,
    Par1: Parser<Input = Par0::Input>,
    Par0::Output: Combinable<Par1::Output>,
{
    type Input = Par0::Input;
    type Output = <Par0::Output as Combinable<Par1::Output>>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser0
            .parse_stream(input)
            .combine_response(|| self.parser1.parse_stream(input))
    }
}
