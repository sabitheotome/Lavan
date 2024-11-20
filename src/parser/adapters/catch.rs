use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// TODO
///
/// This `struct` is created by the [`TODO`] method on [`TODO`].
/// See its documentation for more.    
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Catch<Par0, Par1> {
    pub(in crate::parser) parser_try: Par0,
    pub(in crate::parser) parser_catch: Par1,
}

#[parser_fn(mut in move)]
fn catch<par0, par1, Out, Val>(mut self: &Catch<par0, par1>) -> Out::WithVal<Val>
where
    par0::Output: Switch<par1::Output, Output = Out>,
    Out: Fallible<Value: Fallible<Value = Val>>,
    Out::WithVal<Val>: Response<Value = Val, Error = Out::Error>,
{
    parser![use [not(mut in move)] => self.parser_try]
        .or(parser![use [not(mut in move)] => self.parser_catch])
        .persist()
        .parse_once(input)
}
