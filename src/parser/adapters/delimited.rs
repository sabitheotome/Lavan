use std::marker::PhantomData;

use crate::parser::prelude::*;
use crate::parser::sources::any_eq;
use crate::output::prelude::*;
use crate::input::prelude::*;

use super::ignore::Discard;

/// A util parser for expecting opening and closing delimiters around
///
/// This `struct` is created by the [`Parser::delimited`] method on [`Parser`].
/// See its documentation for more.
pub struct Delimited<Par, Del0, Del1> {
    parser: Par,
    open: Del0,
    close: Del1,
}

impl<Par, Del0, Del1, First, Second> Delimited<Par, Del0, Del1>
where
    Par: Parser,
    Del0: Parser<Input = Par::Input>,
    Del1: Parser<Input = Par::Input>,
    Del0::Output: Combine<Par::Output, Output = First>,
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

    pub(crate) fn discard_delimiters(self) -> Delimited<Par, Discard<Del0>, Discard<Del1>>
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

impl<Par, Del0, Del1, First, Second> Parser for Delimited<Par, Del0, Del1>
where
    Par: Parser,
    Del0: Parser<Input = Par::Input>,
    Del1: Parser<Input = Par::Input>,
    Del0::Output: Combine<Par::Output, Output = First>,
    First: Combine<Del1::Output, Output = Second>,
    Second: Response,
{
    type Input = Par::Input;
    type Output = Second;

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        self.open
            .as_ref()
            .and(self.parser.as_ref())
            .and(self.close.as_ref())
            .next(input)
    }
}
