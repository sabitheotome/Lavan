use crate::parser::prelude::*;
use crate::response::prelude::*;
use crate::stream::traits::Stream;
use crate::Self::Input::traits::Self::Input;
use std::marker::PhantomData;

pub struct TryWith<Par0, Par1, Fun> {
    pub(super) parser0: Par0,
    pub(super) parser1: Par1,
    pub(super) function: Fun,
}

impl<Par0, Par1, Fun> Parser for TryWith<Par0, Par1, Fun>
where
    Par0: Parser,
    Par1: Parser,
    Fun: FnMut(Par0::Output, Par1::Output) -> Result<Par0::Output, Par0::Output>,
{
    type Input = Par0::Input;
    type Output = Par0::Output;

    #[inline]
    fn parse_stream(&mut self, input: &mut Self::Input) -> Self::Output {
        let value0 = self.parser0.parse(input)?;

        let offset = input.offset().clone();
        match self.parser1.parse(input) {
            Ok(value1) => match (self.function)(value0, value1) {
                Ok(value) => Ok(value),
                Err(value) => {
                    input.set(offset);
                    Ok(value)
                }
            },
            Err(_) => {
                input.set(offset);
                Ok(value0)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Slice<'a, Par> {
    pub(super) parser: Par,
    pub(super) __marker: PhantomData<&'a ()>,
}

impl<'a, Par> Parser for Slice<'a, Par>
where
    Par: Parser<Output = ()>,
    Par::Input: 'a,
{
    type Input = Par::Input;
    type Output = Par::Output as Triable<Self::Input as Stream>::Slice<'a>;

    #[inline]
    fn parse_stream(&mut self, input: &mut Self::Input) -> Self::Output {
        let start = input.offset();
        self.parser.parse_stream(input)?;
        let end = input.offset();

        Ok(input.slice(start, end))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Owned<Par> {
    pub(super) parser: Par,
}

impl<Par> Parser for Owned<Par>
where
    Par: Parser,
    Par::Output: ToOwned,
{
    type Input = Par::Input;
    type Output = <Par::Output as ToOwned>::Owned;

    #[inline(always)]
    fn parse_stream(&mut self, input: &mut Self::Input) -> Self::Output {
        Ok(self.parser.parse(input)?.to_owned())
    }
}

#[derive(Debug)]
pub struct ParseStr<Par, Out> {
    pub(super) parser: Par,
    pub(super) _marker: PhantomData<Out>,
}

impl<'a, Par, Out> Parser for ParseStr<Par, Out>
where
    Par: Parser<Output = &'a str>,
    Out: std::str::FromStr,
{
    type Input = Par::Input;
    type Output = Result<Out, Out::Err>;

    #[inline(always)]
    fn parse_stream(&mut self, input: &mut Self::Input) -> Self::Output {
        Ok(self.parser.parse(input)?.parse())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Unwrapped<Par> {
    pub(super) parser: Par,
}

impl<Par, Out, Err> Parser for Unwrapped<Par>
where
    Par: Parser<Output = Result<Out, Err>>,
    Err: std::fmt::Debug,
{
    type Input = Par::Input;
    type Output = Out;

    #[inline(always)]
    fn parse_stream(&mut self, input: &mut Self::Input) -> Self::Output {
        todo!()
        //Ok(self.parser.parse_stream(input)?.unwrap())
    }
}
