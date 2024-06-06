use crate::input::prelude::*;
use std::ops::ControlFlow;

pub trait Response {
    type Value;
    type Error;
    type WithVal<Val>: Response<Error = Self::Error>;
    type WithErr<Err>: Response<Value = Self::Value>;

    fn from_value(collector: Self::Value) -> Self;

    fn from_error(error: Self::Error) -> Self;

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value>;

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val;

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err;

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>;

    fn on_ok<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(),
    {
        match self.control_flow() {
            ControlFlow::Continue(ok) => {
                f();
                Self::from_value(ok)
            }
            ControlFlow::Break(err) => Self::from_error(err),
        }
    }

    fn on_err<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(),
    {
        match self.control_flow() {
            ControlFlow::Continue(ok) => Self::from_value(ok),
            ControlFlow::Break(err) => {
                f();
                Self::from_error(err)
            }
        }
    }
}

pub trait ValueFunctor: Response {
    fn unwrap(self) -> Self::Value
    where
        Self::Error: std::fmt::Debug;
}

pub trait ErrorFunctor: Response {
    fn unwrap_err(self) -> Self::Error
    where
        Self::Value: std::fmt::Debug;
}

pub trait Attachable: Response {
    type Output<V>: Ignorable;
    fn attach_to_response<V>(self, value: impl FnOnce() -> V) -> Self::Output<V>;
}

pub trait ErrAttachable: Response {
    type Output<E>: ErrIgnorable;
    fn attach_err_to_response<E>(self, value: impl FnOnce() -> E) -> Self::Output<E>;
}

pub trait Ignorable: Response {
    type Output: Attachable;
    fn ignore_response(self) -> Self::Output;
}

pub trait ErrIgnorable: Response {
    type Output: ErrAttachable;
    fn ignore_err_response(self) -> Self::Output;
}

pub trait Mappable<Fun>: Response {
    type Output: Response;
    fn map_response(self, f: &Fun) -> Self::Output;
}

impl<Fun, Val0, Val1, T> Mappable<Fun> for T
where
    Fun: Fn(Val0) -> Val1,
    T: ValueFunctor<Value = Val0>,
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
    T: ErrorFunctor<Error = Err0>,
{
    type Output = T::WithErr<Err1>;

    fn err_map_response(self, f: &Fun) -> Self::Output {
        self.map_err(f)
    }
}

pub trait Fallible: Response {
    type Infallible: Response<Value = Self::Value>;
    type Optional: Response;

    fn optional(self) -> Self::Optional
    where
        Self: Sized;
}

pub trait Filterable: ValueFunctor {
    type Output: Fallible;

    fn filter_response(self, predicate: impl FnOnce(&Self::Value) -> bool) -> Self::Output;
}

// TODO: Possibly planned to be renamed
pub trait FilterableWithErr<Err>: ValueFunctor {
    type Output: Fallible<Error = Err>;

    fn filter_response_or_else(
        self,
        predicate: impl FnOnce(&Self::Value) -> bool,
        error: impl FnOnce() -> Err,
    ) -> Self::Output;
}

pub trait Apply<F> {
    type Output: Response;
    fn apply(self, f: &F) -> Self::Output;
}

pub trait Combine<Res> {
    type Output: Response;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Res;
}

pub trait Switch<Other> {
    type Output: Response;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Other;
}
