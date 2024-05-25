use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::parser::prelude::*;

/// A parser for attaching the span of offsets to the reponse
///
/// This `struct` is created by the [`Parser::spanned`] method on [`Parser`].
/// See its documentation for more.
pub struct Spanned<Par> {
    parser: Par,
}

impl<Par> Spanned<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: Parser,
        Par::Input: ScannerSpan,
        Par::Output: ValueFunctor,
    {
        Self { parser }
    }
}

impl<Par, Val> Parser for Spanned<Par>
where
    Par: Parser,
    Par::Input: ScannerSpan,
    Par::Output: ValueFunctor<Value = Val>,
{
    type Input = Par::Input;
    type Output = <Par::Output as Response>::WithVal<(Val, <Par::Input as ScannerSpan>::Span)>;

    fn next(&self, input: &mut Self::Input) -> Self::Output {
        let start = input.span_offset();
        self.parser
            .next(input)
            .map(|value| (value, input.span_since(start)))
    }
}
