use crate::{prelude::*, stream::traits::StrStream};

pub fn word<'a, I: 'a + StrStream<'a, Offset = usize>>(
    word: &'a str,
) -> impl 'a + Parser<Input = I, Output = bool> {
    take(word.len()).eq(word).ignore()
}

pub fn keyword<'a, I: 'a + StrStream<'a, Offset = usize>>(
    keyword: &'a str,
) -> impl 'a + Parser<Input = I, Output = bool> {
    word(keyword).and(
        any_if(|c: &char| c.is_alphanumeric() || *c == '_')
            .auto_bt()
            .ignore(),
    )
}

pub fn identifier<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(char::is_ascii_alphabetic)
        .or(any_eq('_'))
        .ignore()
        .and(
            any_if(char::is_ascii_alphanumeric)
                .or(any_eq('_'))
                .ignore()
                .repeat(),
        )
        .slice()
}

pub fn whitespace<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = ()> {
    any_if(char::is_ascii_whitespace).ignore().repeat()
}

pub fn decimal_integer<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(char::is_ascii_digit).ignore().repeat_min(1).slice()
}
