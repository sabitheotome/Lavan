use std::convert::Infallible;
use std::ops::ControlFlow;

use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// TODO
///
/// This `struct` is created by the [`TODO`] method on [`TODO`].
/// See its documentation for more.    
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct NeverFails<Par, Err>(pub(crate) Par, pub(crate) std::marker::PhantomData<Err>);

impl<Par, Err> NeverFails<Par, Err> {
    pub fn inner(self) -> Par {
        self.0
    }
}

#[parser_fn]
fn never_fails<par, Err>(self: &NeverFails<par, Err>) -> err![par<Err>]
where
    par: Response<Error = Infallible>,
{
    Self::Output::from_value(match parse![self.0].control_flow() {
        ControlFlow::Continue(val) => val,
        ControlFlow::Break(_infallible) => unreachable!(),
    })
}
