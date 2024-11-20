use crate::input::prelude::*;
use crate::parser::prelude::*;
use crate::response::prelude::*;

/// A parser for combining two parsers through [`Combinable`].
///
/// This `struct` is created by the [`Parser::and`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy)]
pub struct And<Par0, Par1> {
    pub(in crate::parser) parser0: Par0,
    pub(in crate::parser) parser1: Par1,
}

#[parser_fn]
fn and<par0, par1>(self: &And<par0, par1>) -> <par0::Output as Combine<par1::Output>>::Output
where
    par0::Output: Combine<par1::Output>,
{
    parse![self.parser0].combine(|| parse![self.parser1])
}

/*
#[cfg(feature = "either")]
mod either {
    use super::super::map_err::FnMapErr;
    use super::And;
    use derive_ctor::ctor;

    use crate::input::prelude::*;
    use crate::response::prelude::*;
    use crate::parser::prelude::*;

    use either::Either;

    impl<Par0, Par1> And<Par0, Par1> {
        pub const fn either_err<Input>(
            self,
        ) -> And<
            FnMapErr<Par0, err![Par0], Either<err![Par0], err![Par1]>>,
            FnMapErr<Par1, err![Par1], Either<err![Par0], err![Par1]>>,
        >
        where
            Input: Stream,
            Par0: Parser<Input>,
            Par1: Parser<Input>,
            Par0::Output: ErrMappable<fn(err![Par0]) -> err![Par1]>,
            FnMapErr<Par0, err![Par0], Either<err![Par0], err![Par1]>>: Parser<Input>,
            FnMapErr<Par1, err![Par1], Either<err![Par0], err![Par1]>>: Parser<Input>,
        {
            And {
                parser0: self
                    .parser0
                    .map_err(either::Either::<err![Par0], err![Par1]>::Left),
                parser1: self
                    .parser1
                    .map_err(either::Either::<err![Par0], err![Par1]>::Right),
            }
        }
    }
}
*/
