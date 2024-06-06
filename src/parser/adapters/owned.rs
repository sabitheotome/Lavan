use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser for converting the value of the response into its Owned version
///
/// This `struct` is created by the [`Parser::owned`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Owned<Par> {
    parser: Par,
}

impl<Par> Owned<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Operator,
        Par::Response: ValueFunctor,
        <Par::Response as Response>::Value: std::borrow::ToOwned,
    {
        Self { parser }
    }
}

impl<Par, Val> Operator for Owned<Par>
where
    Par: Operator,
    Par::Response: ValueFunctor<Value = Val>,
    Val: std::borrow::ToOwned,
{
    type Scanner = Par::Scanner;
    type Response = <Par::Response as Response>::WithVal<<Val as std::borrow::ToOwned>::Owned>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser.parse_next(input).map(|value| value.to_owned())
    }
}
