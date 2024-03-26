use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

// TODO: Documentation
pub type FnMapErr<Par, Val0, Val1> = MapErr<Par, fn(Val0) -> Val1>;

/// A parser for mapping the [`Response::Error`] through [`ErrMappable`]
///
/// This `struct` is created by the [`Parser::map_err`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
pub struct MapErr<Par, Fun> {
    parser: Par,
    function: Fun,
}

impl<Par, Fun> MapErr<Par, Fun> {
    pub(crate) fn new(parser: Par, function: Fun) -> Self
    where
        Par: Parser,
        Par::Output: ErrMappable<Fun>,
    {
        MapErr { parser, function }
    }
}

impl<Par, Fun> Parser for MapErr<Par, Fun>
where
    Par: Parser,
    Par::Output: ErrMappable<Fun>,
{
    type Input = Par::Input;
    type Output = <Par::Output as ErrMappable<Fun>>::Output;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        self.parser
            .parse_stream(input)
            .err_map_response(&self.function)
    }
}
