use crate::{prelude::*, stream::traits::StrStream};

pub fn string<'a, I: 'a + StrStream<'a>>(
    quotation_mark: char,
) -> impl 'a + Parser<Input = I, Output = Option<&'a str>> {
    any_ne(quotation_mark)
        .discard()
        .repeat()
        .delimited(quotation_mark, quotation_mark)
        .discard()
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
            .discard(),
    )
}

pub fn ascii_keyword<'a, I: 'a + StrStream<'a, Offset = usize>>(
    keyword: &'a str,
) -> impl 'a + Parser<Input = I, Output = Option<&'a str>> {
    word(keyword).and(
        any_if(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
            .auto_bt()
            .discard(),
    )
}

pub fn identifier<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(|c: &char| c.is_alphabetic())
        .or(any_eq('_'))
        .discard()
        .and(
            any_if(|c: &char| c.is_alphanumeric())
                .or(any_eq('_'))
                .discard()
                .repeat(),
        )
        .slice()
}

pub fn ascii_identifier<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>>
{
    any_if(char::is_ascii_alphabetic)
        .or(any_eq('_'))
        .discard()
        .and(
            any_if(char::is_ascii_alphanumeric)
                .or(any_eq('_'))
                .discard()
                .repeat(),
        )
        .slice()
}

pub fn trim<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = ()> {
    any_if(|c: &char| c.is_whitespace()).discard().repeat()
}

pub fn ascii_trim<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = ()> {
    any_if(char::is_ascii_whitespace).discard().repeat()
}

pub fn decimal_float<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(char::is_ascii_digit)
        .discard()
        .repeat()
        .and(any_eq('.').discard())
        .and(any_if(char::is_ascii_digit).discard().repeat_min(1))
        .slice()
}

pub fn hexadecimal_integer<'a, I: StrStream<'a>>(
) -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(char::is_ascii_hexdigit)
        .discard()
        .repeat_min(1)
        .slice()
}

pub fn decimal_integer<'a, I: StrStream<'a>>() -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(char::is_ascii_digit).discard().repeat_min(1).slice()
}

pub fn radix_integer<'a, I: StrStream<'a>>(
    radix: u32,
) -> impl Parser<Input = I, Output = Option<&'a str>> {
    any_if(move |c: &char| c.is_digit(radix))
        .discard()
        .repeat_min(1)
        .slice()
}
