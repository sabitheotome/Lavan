use crate::parser::prelude::*;
use crate::parser::sources::any_eq;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

/// A util parser for expecting opening and closing delimiters around
///
/// This `struct` is created by the [`Parser::delimited`] method on [`Parser`].
/// See its documentation for more.
pub struct Delimited<Par, Del> {
    parser: Par,
    open: Del,
    close: Del,
}

impl<Par, Del> Delimited<Par, Del> {
    pub(crate) fn new<First, Second>(parser: Par, open: Del, close: Del) -> Self
    where
        Par: Parser,
        Par::Input: Stream<Item = Del>,
        Del: PartialEq,
        Option<Del>: Combinable<Par::Output, Output = First>,
        First: Combinable<Option<Del>, Output = Second>,
        Second: Response,
    {
        Self {
            parser,
            open,
            close,
        }
    }
}

impl<Par, Del, First, Second> Parser for Delimited<Par, Del>
where
    Par: Parser,
    Par::Input: Stream<Item = Del>,
    Del: PartialEq,
    Option<Del>: Combinable<Par::Output, Output = First>,
    First: Combinable<Option<Del>, Output = Second>,
    Second: Response,
{
    type Input = Par::Input;
    type Output = Second;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        crate::parser::sources::any_if(|e| *e == self.open)
            .and(self.parser.as_ref())
            .and(crate::parser::sources::any_if(|e| *e == self.open))
            .parse_stream(input)
    }
}
