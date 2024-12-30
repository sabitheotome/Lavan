use crate::parser::prelude::internal::*;

/// A parser for converting the value of the response into its Owned version
///
/// This `struct` is created by the [`Parser::owned`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Owned<Par> {
    pub(in crate::parser) parser: Par,
}

#[parser_fn]
fn owned<par, Val>(
    self: &Owned<par>,
) -> <par::Output as Response>::WithVal<<Val as std::borrow::ToOwned>::Owned>
where
    par::Output: ValueResponse<Value = Val>,
    Val: std::borrow::ToOwned,
{
    parse![self.parser].map(|value| value.to_owned())
}
