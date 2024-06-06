use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser for attaching the span of offsets to the reponse
///
/// This `struct` is created by the [`Parser::spanned`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Spanned<Par> {
    parser: Par,
}

impl<Par> Spanned<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Operator,
        Par::Scanner: ScannerSpan,
        Par::Response: ValueFunctor,
    {
        Self { parser }
    }
}

impl<Par, Val> Operator for Spanned<Par>
where
    Par: Operator,
    Par::Scanner: ScannerSpan,
    Par::Response: ValueFunctor<Value = Val>,
{
    type Scanner = Par::Scanner;
    type Response =
        <Par::Response as Response>::WithVal<(Val, <Par::Scanner as ScannerSpan>::Span)>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let start = input.span_offset();
        self.parser
            .parse_next(input)
            .map(|value| (value, input.span_since(start)))
    }
}
