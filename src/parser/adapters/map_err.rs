use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

// TODO: Documentation
pub type FnMapErr<Par, Val0, Val1> = MapErr<Par, fn(Val0) -> Val1>;

/// A parser for mapping the [`Response::Error`] through [`ErrMappable`]
///
/// This `struct` is created by the [`Parser::map_err`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct MapErr<Par, Fun> {
    parser: Par,
    function: Fun,
}

impl<Par, Fun> MapErr<Par, Fun> {
    pub(crate) fn new(parser: Par, function: Fun) -> Self
    where
        Par: Operator,
        Par::Response: ErrMappable<Fun>,
    {
        MapErr { parser, function }
    }
}

impl<Par, Fun> Operator for MapErr<Par, Fun>
where
    Par: Operator,
    Par::Response: ErrMappable<Fun>,
{
    type Scanner = Par::Scanner;
    type Response = <Par::Response as ErrMappable<Fun>>::Output;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.parser
            .parse_next(input)
            .err_map_response(&self.function)
    }
}
