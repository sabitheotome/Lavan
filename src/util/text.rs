use crate::prelude::*;
use adapters::*;
use lavan_proc_macros::{parser_fn, source_parser};

pub fn string<'a, I: 'a + StrStream<'a>>(
    quotation_mark: char,
) -> impl 'a + ParseOnce<I, Output = Option<&'a str>> {
    any_ne(quotation_mark)
        .del()
        .repeat()
        .delimited(any_eq(quotation_mark), any_eq(quotation_mark))
        .del()
        .slice()
}

pub mod ascii {
    use super::*;
    use adapters::ascii::*;

    #[source_parser]
    pub fn alphanumeric(self: &AsciiAlphanumeric) -> Option<char>
    where
        INPUT: Stream<Item = char>,
    {
        implement! {
            eval! {
                any_if(char::is_ascii_alphanumeric)
            }
        }

        AsciiAlphanumeric
    }

    #[source_parser]
    pub fn alphabetic(self: &AsciiAlphabetic) -> Option<char>
    where
        INPUT: Stream<Item = char>,
    {
        implement! {
            eval! {
                any_if(char::is_ascii_alphabetic)
            }
        }

        AsciiAlphabetic
    }

    #[source_parser]
    pub fn trim(self: &AsciiTrim) -> ()
    where
        INPUT: Stream<Item = char>,
    {
        implement! {
            eval! {
                any_if(char::is_ascii_whitespace).del().repeat()
            }
        }

        AsciiTrim
    }

    #[source_parser]
    pub fn identifier(self: &AsciiIdent) -> Option<INPUT::Slice>
    where
        INPUT: StreamSlice<Item = char>,
    {
        implement! {
            eval! {
                any_if(char::is_ascii_alphabetic)
                    .or('_')
                    .del()
                    .and(
                        any_if(char::is_ascii_alphanumeric)
                            .or('_')
                            .del()
                            .repeat_eoi(),
                    )
                    .slice()
            }
        }

        AsciiIdent
    }

    #[source_parser]
    pub fn hex_int(self: &HexInt) -> Option<INPUT::Slice>
    where
        INPUT: StreamSlice<Item = char>,
    {
        implement! {
            eval! {
                any_if(char::is_ascii_hexdigit)
                    .del()
                    .repeat_min(1)
                    .slice()
            }
        }

        HexInt
    }

    #[source_parser]
    pub fn decimal_int(self: &DecInt) -> Option<INPUT::Slice>
    where
        INPUT: StreamSlice<Item = char>,
    {
        implement! {
            eval! {
                any_if(char::is_ascii_digit).del().repeat_min(1).slice()
            }
        }

        DecInt
    }

    #[source_parser]
    pub fn decimal_float(self: &DecFloat) -> Option<INPUT::Slice>
    where
        INPUT: StreamSlice<Item = char>,
    {
        implement! {
            eval! {
                any_if(char::is_ascii_digit)
                .del()
                .repeat()
                .and(any_eq('.').del())
                .and(any_if(char::is_ascii_digit).del().repeat_min(1))
                .slice()
            }
        }

        DecFloat
    }
}

pub mod utf {
    use super::*;

    pub fn alphanumeric<'a, I: 'a + StrStream<'a>>() -> impl 'a + ParseOnce<I, Output = bool> {
        any_if(|c: &char| c.is_alphanumeric()).del()
    }

    pub fn alphabetic<'a, I: 'a + StrStream<'a>>() -> impl 'a + ParseOnce<I, Output = bool> {
        any_if(|c: &char| c.is_alphanumeric()).del()
    }

    pub fn identifier<'a, I: 'a + StrStream<'a>>() -> impl ParseOnce<I, Output = Option<&'a str>> {
        any_if(|c: &char| c.is_alphabetic())
            .or(any_eq('_'))
            .del()
            .and(
                any_if(|c: &char| c.is_alphanumeric())
                    .or(any_eq('_'))
                    .del()
                    .repeat(),
            )
            .slice()
    }

    pub fn trim<'a, I: StrStream<'a>>() -> impl ParseOnce<I, Output = ()> {
        any_if(|c: &char| c.is_whitespace()).del().repeat()
    }

    pub fn int_seq<'a, I: 'a + StrStream<'a>>(
        radix: u32,
    ) -> impl ParseOnce<I, Output = Option<&'a str>> {
        any_if(move |c: &char| c.is_digit(radix))
            .del()
            .repeat_min(1)
            .slice()
    }
}

pub mod adapters {
    pub mod ascii {
        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct AsciiAlphanumeric;

        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct AsciiAlphabetic;

        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct AsciiIdent;

        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct AsciiTrim;
        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct HexInt;

        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct DecInt;

        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct DecFloat;
    }
    pub mod utf {
        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct UtfAlphanumeric;

        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct UtfAlphabetic;

        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct UtfIdent;

        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct UtfTrim;
        /// TODO
        ///
        /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
        /// See its documentation for more.
        #[must_use = "Parsers are lazy and do nothing unless consumed"]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy)]
        pub struct IntSeq;
    }

    /// TODO
    ///
    /// This `struct` is created by the [`TODO`] method on [`TODO`](crate::TODO).
    /// See its documentation for more.
    #[must_use = "Parsers are lazy and do nothing unless consumed"]
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy)]
    pub struct Surrounded;
}

mod impls {
    use super::*;

    #[parser_fn]
    fn char<'a>(self: &char) -> bool
    where
        INPUT: Stream<Item = Self>,
    {
        any_eq(when! { move => self, _ => *self, })
            .del()
            .parse_once(input)
    }

    #[parser_fn]
    fn str<'a>(self: &&'a str) -> bool
    where
        INPUT: StreamSlice<Slice = Self>,
    {
        take(self.len())
            .eq(when! { move => self, _ => *self, })
            .del()
            .parse_once(input)
    }
}
