use crate::parser::prelude::internal::*;

/// A parser for generating auto-backtracking variants with another parsers
///
/// This `struct` is created by the [`Parser::try_with`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct TryWith<Par0, Par1, Fun> {
    pub(in crate::parser) parser0: Par0,
    pub(in crate::parser) parser1: Par1,
    pub(in crate::parser) function: Fun,
}

#[parser_fn]
fn try_with<par0, par1, Fun>(self: &TryWith<par0, par1, Fun>) -> par0::Output
where
    par0::Output: Response<Value = val![par1]>,
    par1::Output: Response<Error = err![par0]>,
    Fun: Fn(val![par0], val![par1]) -> std::ops::ControlFlow<val![par0], val![par1]>,
{
    use std::ops::ControlFlow::{Break, Continue};
    let value0 = tryexpr!(parse![self.parser0]);
    let mut save_state = input.savestate();
    match parse![self.parser1].control_flow() {
        Continue(value1) => match (self.function)(value0, value1) {
            Continue(new) => par0::Output::from_value(new),
            Break(original) => {
                input.backtrack(save_state);
                par0::Output::from_value(original)
            }
        },
        Break(_error) => {
            input.backtrack(save_state);
            par0::Output::from_value(value0)
        }
    }
}

/*
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
}*/
