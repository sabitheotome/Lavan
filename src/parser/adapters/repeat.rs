use crate::input::prelude::*;
use crate::output::prelude::*;
use crate::output::util::try_op;
use crate::parser::prelude::*;
use std::marker::PhantomData;
use std::ops::ControlFlow;

/// A parser for repeatition and collection
///
/// This `struct` is created by the [`Parser::repeat`] method on [`Parser`].
/// See its documentation for more.
#[must_use = "Parsers are lazy and do nothing unless consumed"]
#[derive(Debug, Clone, Copy, ParserAdapter)]
pub struct Repeater<Par, Mod, Col = ()> {
    parser: Par,
    mode: Mod,
    collector: PhantomData<Col>,
}

pub mod mode {
    use std::marker::PhantomData;

    #[derive(Clone, Copy, Debug)]
    pub struct UntilErr(pub(crate) ());

    #[derive(Clone, Copy, Debug)]
    pub struct UntilEOI(pub(crate) ());

    #[derive(Clone, Copy, Debug)]
    pub struct Minimum(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct MinimumEOI(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct Maximum(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct Exact(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct Inter<Mod, Int>(pub(crate) Mod, pub(crate) Int);
}

use mode::*;

// TODO: Documentation
pub type Repeat<Par, Col = ()> = Repeater<Par, UntilErr, Col>;
// TODO: Documentation
pub type RepeatEOI<Par, Col = ()> = Repeater<Par, UntilEOI, Col>;
// TODO: Documentation
pub type RepeatMin<Par, Col = ()> = Repeater<Par, Minimum, Col>;
// TODO: Documentation
pub type RepeatMinEOI<Par, Col = ()> = Repeater<Par, MinimumEOI, Col>;
// TODO: Documentation
pub type RepeatMax<Par, Col = ()> = Repeater<Par, Maximum, Col>;
// TODO: Documentation
pub type RepeatExact<Par, Col = ()> = Repeater<Par, Exact, Col>;

impl<Par, Mod> Repeater<Par, Mod> {
    #[inline]
    pub(crate) fn new(parser: Par, mode: Mod) -> Self
    where
        Par: Operator,
    {
        Self {
            parser,
            mode,
            collector: PhantomData,
        }
    }

    #[inline]
    pub fn collect<T>(self) -> Repeater<Par, Mod, T>
    where
        T: Default + Extend<<Par::Response as Response>::Value>,
        Par: Operator,
        Par::Response: ValueFunctor,
    {
        Repeater {
            parser: self.parser,
            mode: self.mode,
            collector: PhantomData,
        }
    }

    #[inline]
    pub fn to_vec(self) -> Repeater<Par, Mod, Vec<<Par::Response as Response>::Value>>
    where
        Par: Operator,
        Par::Response: ValueFunctor,
    {
        Repeater {
            parser: self.parser,
            mode: self.mode,
            collector: PhantomData,
        }
    }
}

// Non-interspersed

impl<Par, Col, Out> Operator for Repeat<Par, Col>
where
    Par: Operator<Response = Out>,
    Col: Default + Extend<Out::Value>,
    Out: Fallible,
    Out::WithVal<Col>: Fallible<Value = Col>,
{
    type Scanner = Par::Scanner;
    type Response = <Out::WithVal<Col> as Fallible>::Infallible;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        loop {
            match self.parser().auto_bt().parse_next(input).control_flow() {
                ControlFlow::Continue(val) => collector.extend([val]),
                ControlFlow::Break(_) => {
                    return <Self::Response as Response>::from_value(collector)
                }
            }
        }
    }
}

impl<Par, Col, Out> Operator for RepeatEOI<Par, Col>
where
    Par: Operator<Response = Out>,
    Col: Default + Extend<Out::Value>,
    Out: Response,
    Out::WithVal<Col>: Response<Value = Col, Error = Out::Error>,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        loop {
            if let None = input.peekable().peek() {
                return Self::Response::from_value(collector);
            }
            collector.extend([try_op!(self.parser().parse_next(input))]);
        }
    }
}

impl<Par, Col, Out> Operator for RepeatMax<Par, Col>
where
    Par: Operator<Response = Out>,
    Col: Default + Extend<Out::Value>,
    Out: Fallible,
    Out::WithVal<Col>: Fallible<Value = Col, Error = Out::Error>,
{
    type Scanner = Par::Scanner;
    type Response = <Out::WithVal<Col> as Fallible>::Infallible;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        for _ in 0..self.mode.0 {
            match self.parser().auto_bt().parse_next(input).control_flow() {
                ControlFlow::Continue(val) => collector.extend([val]),
                ControlFlow::Break(_) => break,
            }
        }
        <Self::Response as Response>::from_value(collector)
    }
}

impl<Par, Col, Out> Operator for RepeatExact<Par, Col>
where
    Par: Operator<Response = Out>,
    Col: Default + Extend<Out::Value>,
    Out: Response,
    Out::WithVal<Col>: Response<Value = Col, Error = Out::Error>,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        for _ in 0..self.mode.0 {
            collector.extend([try_op!(self.parser().parse_next(input))])
        }
        Self::Response::from_value(collector)
    }
}

impl<Par, Col, Out> Operator for RepeatMin<Par, Col>
where
    Par: Operator<Response = Out>,
    Col: Default + Extend<Out::Value>,
    Out: Fallible,
    Out::WithVal<Col>: Fallible<Value = Col, Error = Out::Error>,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        for _ in 0..self.mode.0 {
            collector.extend([try_op!(self.parser().parse_next(input))])
        }
        loop {
            match self.parser().auto_bt().parse_next(input).control_flow() {
                ControlFlow::Continue(val) => collector.extend([val]),
                ControlFlow::Break(_) => return Self::Response::from_value(collector),
            }
        }
    }
}

impl<Par, Col, Out> Operator for RepeatMinEOI<Par, Col>
where
    Par: Operator<Response = Out>,
    Col: Default + Extend<Out::Value>,
    Out: Fallible,
    Out::WithVal<Col>: Fallible<Value = Col, Error = Out::Error>,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        for _ in 0..self.mode.0 {
            collector.extend([try_op!(self.parser().parse_next(input))])
        }
        loop {
            match self.parser().parse_next(input).control_flow() {
                ControlFlow::Continue(value) => {
                    collector.extend([value]);
                }
                ControlFlow::Break(error) => {
                    if let None = input.next() {
                        return Self::Response::from_value(collector);
                    } else {
                        return Self::Response::from_error(error);
                    }
                }
            }
        }
    }
}

// Interspersed

impl<Par, Int, Col, Out> Operator for Repeater<Par, Inter<UntilErr, Int>, Col>
where
    Par: Operator<Response = Out>,
    Int: Operator<Scanner = Par::Scanner>,
    Col: Default + Extend<Out::Value>,
    Out: Fallible,
    Out::WithVal<Col>: Response<Value = Col, Error = Out::Error>,
    Int::Response: Fallible,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        let Inter(UntilErr(()), ref int) = self.mode;
        // first iteration
        match self.parser().auto_bt().parse_next(input).control_flow() {
            ControlFlow::Continue(val) => collector.extend([val]),
            ControlFlow::Break(err) => return Self::Response::from_value(collector),
        }
        loop {
            // breaks if separator was found
            if let ControlFlow::Break(err) = int.as_ref().auto_bt().parse_next(input).control_flow()
            {
                break;
            }
            // expects main parser
            collector.extend([try_op!(self.parser().parse_next(input))])
        }
        Self::Response::from_value(collector)
    }
}

impl<Par, Int, Col, Out> Operator for Repeater<Par, Inter<UntilEOI, Int>, Col>
where
    Par: Operator<Response = Out>,
    Int: Operator<Scanner = Par::Scanner>,
    Col: Default + Extend<Out::Value>,
    Out: Response,
    Out::WithVal<Col>: Response<Value = Col, Error = Out::Error>,
    Int::Response: Fallible,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        let Inter(UntilEOI(()), ref int) = self.mode;
        match self.parser().parse_next(input).control_flow() {
            ControlFlow::Continue(value) => collector.extend([value]),
            ControlFlow::Break(error) => {
                if let None = input.next() {
                    return Self::Response::from_value(collector);
                } else {
                    return Self::Response::from_error(error);
                }
            }
        }
        loop {
            if let ControlFlow::Break(err) = int.as_ref().auto_bt().parse_next(input).control_flow()
            {
                break;
            }
            match self.parser().parse_next(input).control_flow() {
                ControlFlow::Continue(value) => collector.extend([value]),
                ControlFlow::Break(error) => {
                    if let None = input.next() {
                        return Self::Response::from_value(collector);
                    } else {
                        return Self::Response::from_error(error);
                    }
                }
            }
        }
        Self::Response::from_value(collector)
    }
}

impl<Par, Int, Col, Out> Operator for Repeater<Par, Inter<Maximum, Int>, Col>
where
    Par: Operator<Response = Out>,
    Int: Operator<Scanner = Par::Scanner>,
    Col: Default + Extend<Out::Value>,
    Out: Fallible,
    Out::WithVal<Col>: Response<Value = Col, Error = Out::Error>,
    Int::Response: Fallible,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        let Inter(Maximum(count), ref int) = self.mode;
        match self.parser().auto_bt().parse_next(input).control_flow() {
            ControlFlow::Continue(val) => collector.extend([val]),
            ControlFlow::Break(err) => return Self::Response::from_value(collector),
        }
        for _ in 0..count - 1 {
            if let ControlFlow::Break(err) = int.as_ref().auto_bt().parse_next(input).control_flow()
            {
                break;
            }
            collector.extend([try_op!(self.parser().parse_next(input))])
        }
        Self::Response::from_value(collector)
    }
}

impl<Par, Int, Col, Out> Operator for Repeater<Par, Inter<Exact, Int>, Col>
where
    Par: Operator<Response = Out>,
    Int: Operator<Scanner = Par::Scanner>,
    Col: Default + Extend<Out::Value>,
    Out: Response,
    Out::WithVal<Col>: Response<Value = Col, Error = Out::Error>,
    Int::Response: Fallible<Error = Out::Error>,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        let Inter(Exact(count), ref int) = self.mode;
        collector.extend([try_op!(self.parser().parse_next(input))]);
        for _ in 0..count - 1 {
            try_op!(int.as_ref().auto_bt().parse_next(input));
            collector.extend([try_op!(self.parser().parse_next(input))])
        }
        Self::Response::from_value(collector)
    }
}

impl<Par, Int, Col, Out> Operator for Repeater<Par, Inter<Minimum, Int>, Col>
where
    Par: Operator<Response = Out>,
    Int: Operator<Scanner = Par::Scanner>,
    Col: Default + Extend<Out::Value>,
    Out: Response,
    Out::WithVal<Col>: Response<Value = Col, Error = Out::Error>,
    Int::Response: Fallible<Error = Out::Error>,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        let Inter(Minimum(count), ref int) = self.mode;
        collector.extend([try_op!(self.parser().parse_next(input))]);
        for _ in 0..count - 1 {
            try_op!(int.as_ref().auto_bt().parse_next(input));
            collector.extend([try_op!(self.parser().parse_next(input))])
        }
        loop {
            if let ControlFlow::Break(err) = int.as_ref().auto_bt().parse_next(input).control_flow()
            {
                break;
            }
            collector.extend([try_op!(self.parser().parse_next(input))])
        }
        Self::Response::from_value(collector)
    }
}

impl<Par, Int, Col, Out> Operator for Repeater<Par, Inter<MinimumEOI, Int>, Col>
where
    Par: Operator<Response = Out>,
    Int: Operator<Scanner = Par::Scanner>,
    Col: Default + Extend<Out::Value>,
    Out: Response,
    Out::WithVal<Col>: Response<Value = Col, Error = Out::Error>,
    Int::Response: Fallible<Error = Out::Error>,
{
    type Scanner = Par::Scanner;
    type Response = Out::WithVal<Col>;

    fn parse_next(&self, input: &mut Self::Scanner) -> Self::Response {
        let mut collector = Col::default();
        let Inter(MinimumEOI(count), ref int) = self.mode;
        collector.extend([try_op!(self.parser().parse_next(input))]);
        for _ in 0..count - 1 {
            try_op!(int.as_ref().auto_bt().parse_next(input));
            collector.extend([try_op!(self.parser().parse_next(input))])
        }
        loop {
            if let ControlFlow::Break(err) = int.as_ref().auto_bt().parse_next(input).control_flow()
            {
                break;
            }
            match self.parser().parse_next(input).control_flow() {
                ControlFlow::Continue(value) => collector.extend([value]),
                ControlFlow::Break(error) => {
                    if let None = input.next() {
                        return Self::Response::from_value(collector);
                    } else {
                        return Self::Response::from_error(error);
                    }
                }
            }
        }
        Self::Response::from_value(collector)
    }
}

impl<Par, Mod, Col> Repeater<Par, Mod, Col> {
    #[inline]
    pub fn separated_by<Input, Int>(
        self,
        parser: Int,
    ) -> Repeater<Par, Inter<Mod, Int::Operator>, Col>
    where
        Input: Scanner,
        Int: Parser<Input>,
    {
        Repeater {
            parser: self.parser,
            mode: Inter(self.mode, parser.operator()),
            collector: self.collector,
        }
    }

    #[inline]
    fn parser(&self) -> super::super::adapters::as_ref::AsRef<Par>
    where
        Par: Operator,
    {
        self.parser.as_ref()
    }
}

impl<Par, Col> Repeat<Par, Col> {
    #[inline]
    pub fn until_eoi(self) -> RepeatEOI<Par, Col> {
        Repeater {
            parser: self.parser,
            mode: UntilEOI(()),
            collector: self.collector,
        }
    }
}

impl<Par, Col> RepeatMin<Par, Col> {
    #[inline]
    pub fn until_eoi(self) -> RepeatMinEOI<Par, Col> {
        Repeater {
            parser: self.parser,
            mode: MinimumEOI(self.mode.0),
            collector: self.collector,
        }
    }
}
