use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// A parser for taking another parser by reference
///
/// This `struct` is created by the [`Parser::and`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct AsRef<'a, Par> {
    pub(in crate::parser) parser: &'a Par,
}

#[parser_fn]
fn as_ref<'a, Par>(self: &AsRef<'a, Par>) -> Par::Output
where
    Par: IterativeParserRef<INPUT>,
{
    self.parser.parse_as_ref(input)
}

/// A parser for taking another parser by reference
///
/// This `struct` is created by the [`Parser::and`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct AsMut<'a, Par> {
    pub(crate) parser: &'a mut Par,
}

#[parser_fn]
fn as_mut<'a, Par>(self: &mut AsMut<'a, Par>) -> Par::Output
where
    Par: IterativeParserMut<INPUT>,
{
    self.parser.parse_as_mut(input)
}
