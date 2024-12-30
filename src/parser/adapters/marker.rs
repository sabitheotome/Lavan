use crate::parser::prelude::internal::*;

/// TODO
///
/// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
/// See its documentation for more.    
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct Marker<Par, I>(pub(crate) Par, pub(crate) std::marker::PhantomData<I>);

pub fn mark<Par, I>(par: Par) -> Marker<Par, I> {
    Marker(par, PhantomData)
}

#[parser_fn]
fn marker<par>(self: &Marker<par, INPUT>) -> par::Output {
    parse![self.0]
}

impl<Par: Clone, I> Clone for Marker<Par, I> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<Par: Copy, I> Copy for Marker<Par, I> {}
