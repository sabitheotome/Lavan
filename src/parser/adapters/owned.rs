use crate::parser::prelude::*;
use crate::output::prelude::*;
use crate::input::prelude::*;

/// A parser for converting the value of the response into its Owned version
///
/// This `struct` is created by the [`Parser::owned`] method on [`Parser`].
/// See its documentation for more.
pub struct Owned<Par> {
    parser: Par,
}

impl<Par> Owned<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Output: ValueFunctor,
        <Par::Output as Response>::Value: std::borrow::ToOwned,
    {
        Self { parser }
    }
}

impl<Par, Val> Parser for Owned<Par>
where
    Par: Parser,
    Par::Output: ValueFunctor<Value = Val>,
    Val: std::borrow::ToOwned,
{
    type Input = Par::Input;
    type Output = <Par::Output as Response>::WithVal<<Val as std::borrow::ToOwned>::Owned>;

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .next(input)
            .map(|value| value.to_owned())
    }
}
