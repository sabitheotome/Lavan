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
    pub(crate) fn new<Out0, Out1>(parser0: Par0, parser1: Par1, function: Fun) -> Self
    where
        Par0: Parser<Output = Out0>,
        Par1: Parser<Output = Out1, Input = Par0::Input>,
        Fun: Fn(Out0::Value, Out1::Value) -> std::ops::ControlFlow<Out0::Value, Out0::Value>,
        Out0: Response,
        Out1: Response<Error = Out0::Error>,
    {
        Self {
            parser0,
            parser1,
            function,
        }
    }
}

impl<Par0, Par1, Fun, Out0, Out1> Parser for TryWith<Par0, Par1, Fun>
where
    Par0: Parser<Output = Out0>,
    Par1: Parser<Output = Out1, Input = Par0::Input>,
    Fun: Fn(Out0::Value, Out1::Value) -> std::ops::ControlFlow<Out0::Value, Out0::Value>,
    Out0: Response,
    Out1: Response<Error = Out0::Error>,
{
    type Input = Par0::Input;
    type Output = Out0;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        use std::ops::ControlFlow::{Break, Continue};
        let value0 = try_op!(self.parser0.parse_stream(input));
        let offset = input.offset();
        match self.parser1.parse_stream(input).control_flow() {
            Continue(value1) => match (self.function)(value0, value1) {
                Continue(new) => Out0::from_value(new),
                Break(original) => {
                    *input.offset_mut() = offset;
                    Out0::from_value(original)
                }
            },
            Break(_error) => {
                *input.offset_mut() = offset;
                Out0::from_value(value0)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn doc_example() {
        use crate::prelude::*;
        use HatOrNoHat::*;

        #[derive(PartialEq, Debug, Clone)]
        enum HatOrNoHat {
            No,
            Hat,
            NoHat,
        }

        let input: &[HatOrNoHat] = &[Hat, Hat, No, Hat, Hat, No, Hat, No, Hat, Hat, No];
        let expected_out = [Hat, Hat, NoHat, Hat, NoHat, NoHat, Hat, No];

        let output = any()
            .try_with(any(), |a: HatOrNoHat, b: HatOrNoHat| {
                if a == No && b == Hat {
                    Continue(NoHat)
                } else {
                    Break(a)
                }
            })
            .repeat()
            .to_vec()
            .parse_stream(&mut (input, 0));

        assert_eq!(output.value(), expected_out);
    }
}
