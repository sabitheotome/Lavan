use crate::response::prelude::internal::*;

impl<T, E> Response for Result<T, E> {
    type Value = T;
    type Error = E;
    type Residual = Exception<E>;
    type WithVal<Val> = Result<Val, E>;
    type WithErr<Err> = Result<T, Err>;

    fn from_value(value: Self::Value) -> Self {
        Ok(value)
    }

    fn from_error(error: Self::Error) -> Self {
        Err(error)
    }

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        match self {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(e),
        }
    }

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
        self.map_err(f)
    }

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        self.map(f)
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        self.and_then(f)
    }
}

impl<T, E> ValueResponse for Result<T, E> {
    type VoidVal = Unsure<E>;

    fn void_val(self) -> Self::VoidVal {
        self.map(|_| ()).into()
    }

    fn unwrap(self) -> Self::Value
    where
        Self::Error: std::fmt::Debug,
    {
        self.unwrap()
    }
}

impl<T, E> ErrorResponse for Result<T, E> {
    type VoidErr = Option<T>;

    fn void_err(self) -> Self::VoidErr {
        self.ok()
    }

    fn unwrap_err(self) -> Self::Error
    where
        Self::Value: std::fmt::Debug,
    {
        self.unwrap_err()
    }
}

impl<Val, Err> Fallible for Result<Val, Err> {
    type Infallible = Sure<Self::Value>;
    type Optional = Sure<Option<Val>>;

    fn optional(self) -> Self::Optional {
        Sure(self.ok())
    }
}

impl<Val, Err> PredictOrElse<Err> for Result<Val, Err> {
    type Output = Result<Val, Err>;

    fn predict_or_else(
        self,
        pred: impl FnOnce(&Self::Value) -> bool,
        err: impl FnOnce() -> Err,
    ) -> Self::Output {
        match self {
            Ok(ok) => match pred(&ok) {
                true => Ok(ok),
                false => Err(err()),
            },
            Err(err) => Err(err),
        }
    }
}

impl<Val, Err, Fun, Out> Apply<Fun> for Result<Val, Err>
where
    Unsure<Err>: Combine<Out>,
    Fun: Fn(Val) -> Out,
    Out: Response,
{
    type Output = <Unsure<Err> as Combine<Out>>::Output;
    fn apply(self, f: &Fun) -> Self::Output {
        let option;
        let unsure;
        match self {
            Ok(value) => {
                option = Some(value);
                unsure = Unsure::ok();
            }
            Err(error) => {
                option = None;
                unsure = Unsure::err(error);
            }
        }
        unsure.combine(|| f(option.unwrap()))
    }
}

impl<Val, Err> Combine<()> for Result<Val, Err> {
    type Output = Self;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(),
    {
        let value = self?;
        f();
        Ok(value)
    }
}

impl<Val0, Val1, Err0, Err1> Combine<Result<Val1, Err1>> for Result<Val0, Err0>
where
    Err1: From<Err0>,
{
    type Output = Result<(Val0, Val1), Err1>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Result<Val1, Err1>,
    {
        Ok((self?, f()?))
    }
}

impl<Val0, Val1, Err> Combine<Sure<Val1>> for Result<Val0, Err> {
    type Output = Result<(Val0, Val1), Err>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Sure<Val1>,
    {
        let value = self?;
        Ok((value, f().value()))
    }
}

impl<Val, Err> Combine<Unsure<Err>> for Result<Val, Err> {
    type Output = Result<Val, Err>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Unsure<Err>,
    {
        let value = self?;
        f().into_result()?;
        Ok(value)
    }
}

impl<Val, Err> Switch<()> for Result<Val, Err> {
    type Output = Sure<Option<Val>>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> (),
    {
        match self {
            Ok(value) => Sure(Some(value)),
            Err(_error) => {
                f();
                Sure(None)
            }
        }
    }
}

impl<Val, Err> Switch<bool> for Result<Val, Err> {
    type Output = Result<Option<Val>, Err>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> bool,
    {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(error) => f().then_some(None).ok_or(error),
        }
    }
}

impl<Val, Err> Switch<Option<Val>> for Result<Val, Err> {
    type Output = Result<Val, Err>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Option<Val>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(error) => f().ok_or(error),
        }
    }
}

impl<Val, Err> Switch<Sure<Val>> for Result<Val, Err> {
    type Output = Sure<Val>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Sure<Val>,
    {
        match self {
            Ok(value) => Sure(value),
            Err(_error) => f(),
        }
    }
}

impl<Val, Err0, Err1> Switch<Result<Val, Err1>> for Result<Val, Err0> {
    type Output = Result<Val, (Err0, Err1)>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Result<Val, Err1>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(error0) => f().map_err(|error1| (error0, error1)),
        }
    }
}

impl<Val, Err0, Err1> Switch<Unsure<Err1>> for Result<Val, Err0> {
    type Output = Result<Option<Val>, (Err0, Err1)>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Unsure<Err1>,
    {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(error0) => f()
                .into_result()
                .map(|()| None)
                .map_err(|error1| (error0, error1)),
        }
    }
}
