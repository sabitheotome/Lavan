use crate::parser::prelude::*;
use crate::parser::util::assoc::err;
use crate::response::prelude::*;
use crate::response::util::try_op;
use crate::stream::traits::Stream;

/// TODO
///
/// This `struct` is created by the [`Parser::try_with`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct TryWith<Par0, Par1, Fun> {
    parser0: Par0,
    parser1: Par1,
    function: Fun,
}

impl<Par0, Par1, Fun> TryWith<Par0, Par1, Fun> {
    pub(crate) fn new<Out>(parser0: Par0, parser1: Par1, function: Fun) -> Self
    where
        Par0: Parser<Output = Out>,
        Par1: Parser<Output = Out, Input = Par0::Input>,
        Fun: Fn(Out::Value, Out::Value) -> std::ops::ControlFlow<Out::Value, Out::Value>,
        Out: Response + Fallible,
    {
        Self {
            parser0,
            parser1,
            function,
        }
    }
}

impl<Par0, Par1, Fun, Out> Parser for TryWith<Par0, Par1, Fun>
where
    Par0: Parser<Output = Out>,
    Par1: Parser<Output = Out, Input = Par0::Input>,
    Fun: Fn(Out::Value, Out::Value) -> std::ops::ControlFlow<Out::Value, Out::Value>,
    Out: Response + Fallible,
{
    type Input = Par0::Input;
    type Output = Par0::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        use std::ops::ControlFlow::{Break, Continue};
        let value0 = try_op!(self.parser0.parse_stream(input));
        let offset = input.offset();
        match self.parser1.parse_stream(input).control_flow() {
            Continue(value1) => match (self.function)(value0, value1) {
                Continue(new) => Par0::Output::from_value(new),
                Break(original) => {
                    *input.offset_mut() = offset;
                    Par0::Output::from_value(original)
                }
            },
            Break(_error) => {
                *input.offset_mut() = offset;
                Par0::Output::from_value(value0)
            }
        }
    }
}
