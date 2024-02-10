use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::response::util::try_op;
use crate::stream::traits::Stream;

pub struct Filter<Par, Fun, Res = fn() -> ()> {
    parser: Par,
    function: Fun,
    residual: Res,
}

impl<Par, Fun> Filter<Par, Fun> {
    pub(crate) fn new(parser: Par, function: Fun) -> Self
    where
        Par: Parser,
        Par::Output: Triable,
        Fun: Fn(&<Par::Output as Pseudotriable>::Output) -> bool,
    {
        Self {
            parser,
            function,
            residual: Self::default_residual_supplier,
        }
    }

    fn default_residual_supplier() {}

    pub fn or_else<Res, Out>(self, residual: Res) -> Filter<Par, Fun, Res>
    where
        Par: Parser<Output = Out>,
        Res: Fn() -> Out::Residual,
        Out: Triable,
    {
        Filter {
            parser: self.parser,
            function: self.function,
            residual,
        }
    }
}

impl<Par, Out, Fun, Res> Parser for Filter<Par, Fun, Res>
where
    Par: Parser<Output = Out>,
    Out: Triable,
    Fun: Fn(&Out::Output) -> bool,
    Res: Fn() -> Out::Residual,
{
    type Input = Par::Input;
    type Output = Out;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        let value = try_op!(self.parser.parse_stream(input));
        match (self.function)(&value) {
            true => Out::from_output(value),
            false => Out::from_residual((self.residual)()),
        }
    }
}
