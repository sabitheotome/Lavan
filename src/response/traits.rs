use crate::response::prelude::internal::*;

pub trait Response {
    type Value;
    type Error;
    type Residual: FromErr<Self::Error> + IntoErr<Self::Error>;
    type WithVal<Val>: Response<Error = Self::Error>;
    type WithErr<Err>: Response<Value = Self::Value>;

    fn from_value(collector: Self::Value) -> Self;
    fn from_error(error: Self::Error) -> Self;

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value>
    where
        Self: Sized;

    fn from_residual<R>(residual: R) -> Self
    where
        Self: Sized,
        R: IntoErr<Self::Error>,
    {
        Self::from_error(residual.into_err())
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Value>
    where
        Self: Sized,
    {
        match self.control_flow() {
            ControlFlow::Continue(val) => ControlFlow::Continue(val),
            ControlFlow::Break(err) => ControlFlow::Break(Self::Residual::from_err(err)),
        }
    }

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

pub trait FromErr<O> {
    fn from_err(v: O) -> Self;
}

pub trait IntoErr<O> {
    fn into_err(self) -> O;
}

pub trait ValueResponse: Response //where Self::WithVal<Self::Value>: Response<Value = Self::Value>,
{
    type VoidVal: Attach;

    fn void_val(self) -> Self::VoidVal;

    fn unwrap(self) -> Self::Value
    where
        Self::Error: std::fmt::Debug;
}

pub trait ErrorResponse: Response //where Self::WithErr<Self::Error>: Response<Error = Self::Error>,
{
    type VoidErr: AttachErr;

    fn void_err(self) -> Self::VoidErr;

    fn unwrap_err(self) -> Self::Error
    where
        Self::Value: std::fmt::Debug;
}

pub trait Attach: Response {
    type Output<V>: ValueResponse;
    fn attach_to_response<V>(self, value: impl FnOnce() -> V) -> Self::Output<V>;
}

pub trait AttachErr: Response {
    type Output<E>: ErrorResponse;
    fn attach_err_to_response<E>(self, value: impl FnOnce() -> E) -> Self::Output<E>;
}

pub trait Select<Fun>: Response {
    type Output: Response;
    fn sel(self, f: &Fun) -> Self::Output;
}

impl<Fun, Val0, Val1, T> Select<Fun> for T
where
    Fun: Fn(Val0) -> Val1,
    T: ValueResponse<Value = Val0>,
{
    type Output = T::WithVal<Val1>;

    fn sel(self, f: &Fun) -> Self::Output {
        self.map(f)
    }
}

pub trait SelectErr<Fun>: Response {
    type Output: Response;
    fn sel_err(self, f: &Fun) -> Self::Output;
}

impl<Fun, Err0, Err1, T> SelectErr<Fun> for T
where
    Fun: Fn(Err0) -> Err1,
    T: ErrorResponse<Error = Err0>,
{
    type Output = T::WithErr<Err1>;

    fn sel_err(self, f: &Fun) -> Self::Output {
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

pub trait Predict: ValueResponse {
    type Output: Fallible;

    fn predict(self, predicate: impl FnOnce(&Self::Value) -> bool) -> Self::Output;
}

pub trait PredictOrElse<Err>: ValueResponse {
    type Output: Fallible<Error = Err>;

    fn predict_or_else(
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
