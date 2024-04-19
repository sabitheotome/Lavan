use crate::{prelude::*, stream::traits::StrStream};

pub fn string<'a, I: 'a + StrStream<'a>>(
    quotation_mark: char,
) -> impl 'a + Parser<Input = I, Output = Option<&'a str>> {
    any_ne(quotation_mark)
        .ignore()
        .repeat()
        .delimited(quotation_mark, quotation_mark)
        .ignore()
        .slice()
}

pub fn word<'a, I: 'a + StrStream<'a, Offset = usize>>(
    word: &'a str,
) -> impl 'a + Parser<Input = I, Output = Option<&'a str>> {
    take(word.len()).eq(word)
}

pub fn keyword<'a, I: 'a + StrStream<'a, Offset = usize>>(
    keyword: &'a str,
) -> impl 'a + Parser<Input = I, Output = Option<&'a str>> {
    word(keyword).and(
        any_if(|c: &char| c.is_alphanumeric() || *c == '_')
            .auto_bt()
            .ignore(),
    )
}

pub fn ascii_keyword<'a, I: 'a + StrStream<'a, Offset = usize>>(
    keyword: &'a str,
) -> impl 'a + Parser<Input = I, Output = Option<&'a str>> {
    word(keyword).and(
        any_if(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
            .auto_bt()
            .ignore(),
    )
}

pub fn identifier<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(|c: &char| c.is_alphabetic())
        .or(any_eq('_'))
        .ignore()
        .and(
            any_if(|c: &char| c.is_alphanumeric())
                .or(any_eq('_'))
                .ignore()
                .repeat(),
        )
        .slice()
}

pub fn ascii_identifier<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>>
{
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

pub fn trim<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = ()> {
    any_if(|c: &char| c.is_whitespace()).ignore().repeat()
}

pub fn ascii_trim<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = ()> {
    any_if(char::is_ascii_whitespace).ignore().repeat()
}

pub fn decimal_float<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(char::is_ascii_digit)
        .ignore()
        .repeat()
        .and(any_eq('.').ignore())
        .and(any_if(char::is_ascii_digit).ignore().repeat_min(1))
        .slice()
}

pub fn hexadecimal_integer<'a, I: StrStream<'a>>(
) -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(char::is_ascii_hexdigit)
        .ignore()
        .repeat_min(1)
        .slice()
}

pub fn decimal_integer<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(char::is_ascii_digit).ignore().repeat_min(1).slice()
}

pub fn radix_integer<'a, I: StrStream<'a>>(
    radix: u32,
) -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(move |c: &char| c.is_digit(radix))
        .ignore()
        .repeat_min(1)
        .slice()
}
