use crate::parser::prelude::internal::*;

/// TODO
///
/// This `struct` is created by the [`TODO`] method on [`TODO`].
/// See its documentation for more.    
#[stability::unstable(feature = "unstable_name")]
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct NeverFails<Par, Err>(pub(crate) Par, pub(crate) std::marker::PhantomData<Err>);

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
