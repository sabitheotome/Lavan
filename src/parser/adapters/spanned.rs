use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;

/// TODO
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
        Par::Output: ValueFunctor,
    {
        Self { parser }
    }
}

impl<Par, Val> Parser for Spanned<Par>
where
    Par: Parser,
    Par::Output: ValueFunctor<Value = Val>,
{
    type Input = Par::Input;
    type Output = <Par::Output as Response>::WithVal<(Val, <Self::Input as Stream>::Span)>;

    fn parse_stream(&self, input: &mut Self::Input) -> Self::Output {
        let start = input.offset();
        self.parser.parse_stream(input).map(|value| {
            let end = input.offset();
            let span = input.span(start, end);
            (value, span)
        })
    }
}
