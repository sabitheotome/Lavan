use super::{super::sources::any_eq, del::Del};
use crate::parser::prelude::internal::*;

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
    pub fn del_delims<Input>(self) -> Delimited<Par, Del<Del0>, Del<Del1>>
    where
        Del0: ParseOnce<Input, Output: ValueResponse>,
        Del1: ParseOnce<Input, Output: ValueResponse>,
    {
        Delimited {
            parser: self.parser,
            open: self.open.del(),
            close: self.close.del(),
        }
    }
}
