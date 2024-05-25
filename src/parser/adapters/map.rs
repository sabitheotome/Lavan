use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

// TODO: Documentation
pub type FnMap<Par, Val0, Val1> = Map<Par, fn(Val0) -> Val1>;

/// A parser for mapping the [`Response::Value`] through [`Mappable`]
///
/// This `struct` is created by the [`Parser::map`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct Map<Par, Fun> {
    parser: Par,
    function: Fun,
}

impl<Par, Fun> Map<Par, Fun> {
    pub(crate) fn new(parser: Par, function: Fun) -> Self
    where
        Par: Parser,
        Par::Output: Mappable<Fun>,
    {
        Map { parser, function }
    }
}

impl<Par, Fun> Parser for Map<Par, Fun>
where
    Par: Parser,
    Par::Output: Mappable<Fun>,
{
    type Input = Par::Input;
    type Output = <Par::Output as Mappable<Fun>>::Output;

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.next(input).map_response(&self.function)
    }
}
