use crate::parser::prelude::internal::*;

/// A parser for converting a `str` to `T` where `T: std::str::FromStr`
///
/// This `struct` is created by the [`Parser::parse_str`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct ParseStr<Par, T> {
    pub(in crate::parser) parser: Par,
    pub(in crate::parser) convert_to: PhantomData<T>,
}

#[parser_fn]
fn parse_str<par, T>(
    self: &ParseStr<par, T>,
) -> <par::Output as Apply<fn(&str) -> Result<T, T::Err>>>::Output
where
    par::Output: Apply<fn(&str) -> Result<T, T::Err>>,
    T: std::str::FromStr,
{
    parse![parser![self.parser].then(|str: &str| str.parse::<T>())]
}
