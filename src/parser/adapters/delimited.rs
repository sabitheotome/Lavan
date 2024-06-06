use std::marker::PhantomData;

use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;
use crate::parser::sources::any_eq;

use super::ignore::Discard;

/// A util parser for expecting opening and closing delimiters around
///
/// This `struct` is created by the [`Parser::delimited`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Delimited<Par, Del0, Del1> {
    parser: Par,
    open: Del0,
    close: Del1,
}

impl<Par, Del0, Del1, First, Second> Delimited<Par, Del0, Del1>
where
    Par: Operator,
    Del0: Parser<Par::Scanner>,
    Del1: Parser<Par::Scanner>,
    Del0::Output: Combine<Par::Response, Output = First>,
    First: Combine<Del1::Output, Output = Second>,
    Second: Response,
{
    pub(crate) fn new(parser: Par, open: Del0, close: Del1) -> Self {
        Self {
            parser,
            open,
            close,
        }
    }

    pub(crate) fn discard_delimiters(
        self,
    ) -> Delimited<Par, Discard<Del0::Operator>, Discard<Del1::Operator>>
    where
        Del0::Output: Ignorable,
        Del1::Output: Ignorable,
    {
        Delimited {
            parser: self.parser,
            open: self.open.discard(),
            close: self.close.discard(),
        }
    }
}

impl<Par, Del0, Del1, First, Second> Operator for Delimited<Par, Del0, Del1>
where
    Par: Operator,
    Del0: Operator<Scanner = Par::Scanner>,
    Del1: Operator<Scanner = Par::Scanner>,
    Del0::Response: Combine<Par::Response, Output = First>,
    First: Combine<Del1::Response, Output = Second>,
    Second: Response,
{
    type Scanner = Par::Scanner;
    type Response = Second;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        self.open
            .as_ref()
            .and(self.parser.as_ref())
            .and(self.close.as_ref())
            .parse_next(input)
    }
}
