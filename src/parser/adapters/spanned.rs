use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// A parser for attaching the span of offsets to the reponse
///
/// This `struct` is created by the [`Parser::spanned`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct Spanned<Par> {
    pub(in crate::parser) parser: Par,
}

#[parser_fn]
fn spanned<par, Val>(
    self: &Spanned<par>,
) -> <par::Output as Response>::WithVal<(Val, <INPUT as StreamSpan>::Span)>
where
    INPUT: StreamSpan,
    par::Output: ValueResponse<Value = Val>,
{
    let start = input.span_offset();
    parse![self.parser].map(|value| (value, input.span_since(start)))
}
