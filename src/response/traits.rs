use crate::stream::traits::Stream;
use std::ops::ControlFlow;

pub trait Response {
    type Value;
    type Error;
    type WithVal<Val>: Response;
    type WithErr<Err>: Response;

    fn from_value(collector: Self::Value) -> Self;
    fn from_error(error: Self::Error) -> Self;

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val;

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err;

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>;

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value>;
}

pub trait ValueFunctor: Response {}
pub trait ErrorFunctor: Response {}

pub trait Combinable<Res>: Response
where
    Res: Response,
{
    type Output: Response;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Res;
}

// TODO: Planned to possibly be removed
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

// TODO: Planned to possibly be removed
pub trait Recoverable: Response {
    fn recover_response<Rec, Str>(self, on_residual: Rec, stream: &mut Str) -> Self
    where
        Rec: FnOnce(&mut Str),
        Str: Stream;
}

pub trait Attachable: Response {
    type Output<V>: Ignorable;
    fn attach_to_response<V>(self, value: V) -> Self::Output<V>;
}

pub trait Ignorable: Response {
    type Output: Attachable;
    fn ignore_response(self) -> Self::Output;
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

pub trait Optionable: Recoverable {
    type Output: Response;
    fn opt_response(self) -> Self::Output;
}

pub trait Fallible: Response {
    type Infallible: Response<Value = Self::Value>;
}

pub trait Filterable: ValueFunctor {
    type Output: Fallible;

    fn filter_response(self, predicate: impl FnOnce(&Self::Value) -> bool) -> Self::Output;
}

pub trait FilterableWithErr<Err>: ValueFunctor {
    type Output: Fallible<Error = Err>;

    fn filter_response_or_else(
        self,
        predicate: impl FnOnce(&Self::Value) -> bool,
        error: impl FnOnce() -> Err,
    ) -> Self::Output;
}
