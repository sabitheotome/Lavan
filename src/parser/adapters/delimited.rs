use std::marker::PhantomData;

use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;
use crate::parser::sources::any_eq;

use super::discard::Discard;

/// A util parser for expecting opening and closing delimiters around
///
/// This `struct` is created by the [`Parser::delimited`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Delimited<Par, Del0, Del1> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) open: Del0,
    pub(in crate::parser) close: Del1,
}

#[parser_fn]
fn delimited<par, del0, del1, Combo>(self: &Delimited<par, del0, del1>) -> Combo::Output
where
    del0::Output: Combine<par::Output, Output = Combo>,
    Combo: Combine<del1::Output, Output: Response>,
{
    let open = parser![self.open];
    let middle = parser![self.parser];
    let close = parser![self.close];

    parse![open.and(middle).and(close)]
}

impl<Par, Del0, Del1> Delimited<Par, Del0, Del1> {
    pub(crate) fn discard_delimiters<Input: Scanner>(
        self,
    ) -> Delimited<Par, Discard<Del0>, Discard<Del1>>
    where
        Del0: Parser<Input, Output: ValueResponse>,
        Del1: Parser<Input, Output: ValueResponse>,
    {
        Delimited {
            parser: self.parser,
            open: self.open.discard(),
            close: self.close.discard(),
        }
    }
}
