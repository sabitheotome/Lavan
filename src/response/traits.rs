use std::{ops::ControlFlow, process::Output};

use crate::stream::traits::Stream;

pub trait Response {}

pub trait Pseudodata: Response {
    type Value;
    type WithVal<Val>: Pseudodata;

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val;

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>;
}

pub trait Data: Pseudodata {}

pub trait Exceptional: Response {
    type Error;
    type WithErr<Err>: Exceptional;

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err;
}

pub trait Pure: Response {
    type Value;

    fn pure(value: Self::Value) -> Self;
    fn unwrap(self) -> Self::Value;
}

pub trait Combinable<Res>: Response
where
    Res: Response,
{
    type Output: Response;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Res;
}

pub trait Disjoinable<Res>: Response
where
    Res: Response,
{
    type Output: Response;

    fn disjoin_response<Fun, Rec, Str>(
        self,
        response: Fun,
        recover: Rec,
        stream: &mut Str,
    ) -> Self::Output
    where
        Fun: FnOnce(&mut Str) -> Res,
        Rec: FnOnce(&mut Str),
        Str: Stream;
}

pub trait Recoverable: Response {
    fn recover_response<Rec, Str>(self, on_residual: Rec, stream: &mut Str) -> Self
    where
        Rec: FnOnce(&mut Str),
        Str: Stream;
}

pub trait Ignorable: Response {
    type Output: Response;
    fn ignore_response(self) -> Self::Output;
}

pub trait Mappable<Fun>: Response {
    type Output: Response;
    fn map_response(self, f: &Fun) -> Self::Output;
}

impl<Fun, Val0, Val1, T> Mappable<Fun> for T
where
    Fun: Fn(Val0) -> Val1,
    T: Data<Value = Val0>,
{
    type Output = T::WithVal<Val1>;

    fn map_response(self, f: &Fun) -> Self::Output {
        self.map(f)
    }
}

pub trait ErrMappable<Fun>: Response {
    type Output: Response;
    fn err_map_response(self, f: &Fun) -> Self::Output;
}

impl<Fun, Err0, Err1, T> ErrMappable<Fun> for T
where
    Fun: Fn(Err0) -> Err1,
    T: Exceptional<Error = Err0>,
{
    type Output = T::WithErr<Err1>;

    fn err_map_response(self, f: &Fun) -> Self::Output {
        self.map_err(f)
    }
}

pub trait Optionable: Recoverable {
    type Output: Response;
    fn opt_response(self) -> Self::Output;
}

pub trait Pseudotriable: Response {
    type Output;
    type Residual;

    fn from_output(collector: Self::Output) -> Self;
    fn from_residual(error: Self::Residual) -> Self;
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
}

pub trait Triable: Pseudotriable {
    type Infallible: Pure<Value = Self::Output>;
}
