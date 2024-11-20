use std::ops::ControlFlow::*;

use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// TODO
///
/// This `struct` is created by the [`TODO`] method on [`TODO`].
/// See its documentation for more.    
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct Persist<Par>(pub(crate) Par);

#[parser_fn(mut in move)]
fn persist<par, Val>(mut self: &Persist<par>) -> val![par<Val>]
where
    par::Output: Fallible<Value: Fallible<Value = Val>>,
    val![par<Val>]: Response<Value = Val, Error = err![par]>,
{
    loop {
        let v = tryexpr!(parse![self.0]);
        if let Continue(val) = v.control_flow() {
            return tryok!(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IterativeParser;
    use crate::prelude::any;

    #[test]
    fn a() {
        any().catch(any().del()).parse_once(&mut "".chars());
    }
}
