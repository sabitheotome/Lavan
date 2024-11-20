use crate::response::prelude::*;

impl<T> Response for Option<T> {
    type Value = T;
    type Error = ();
    type Residual = Exception<()>;
    type WithVal<Val> = Option<Val>;
    type WithErr<Err> = Option<T>;

    fn from_value(value: Self::Value) -> Self {
        Some(value)
    }

    fn from_error((): Self::Error) -> Self {
        None
    }

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        match self {
            Some(v) => ControlFlow::Continue(v),
            None => ControlFlow::Break(()),
        }
    }

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        self.map(f)
    }

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
        match self {
            Some(val) => Some(val),
            None => {
                f(());
                None
            }
        }
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        self.and_then(f)
    }
}

impl<T> ValueResponse for Option<T> {
    type VoidVal = bool;

    fn void_val(self) -> Self::VoidVal {
        self.is_some()
    }

    fn unwrap(self) -> Self::Value {
        self.unwrap()
    }
}

impl<Val> AttachErr for Option<Val> {
    type Output<Err> = Result<Val, Err>;

    fn attach_err_to_response<E>(self, value: impl FnOnce() -> E) -> Self::Output<E> {
        self.ok_or_else(value)
    }
}

impl<Fun, Val, Err> SelectErr<Fun> for Option<Val>
where
    Fun: Fn() -> Err,
{
    type Output = Result<Val, Err>;

    fn sel_err(self, f: &Fun) -> Self::Output {
        match self {
            Some(value) => Ok(value),
            None => Err(f()),
        }
    }
}

impl<Val> Fallible for Option<Val> {
    type Infallible = Sure<Self::Value>;
    type Optional = Sure<Self>;

    fn optional(self) -> Self::Optional {
        Sure(self)
    }
}

impl<Val> Predict for Option<Val> {
    type Output = Option<Val>;

    fn predict(self, predicate: impl FnOnce(&Self::Value) -> bool) -> Self::Output {
        self.filter(predicate)
    }
}

impl<Val, Err> PredictOrElse<Err> for Option<Val> {
    type Output = Result<Val, Err>;

    fn predict_or_else(
        self,
        pred: impl FnOnce(&Self::Value) -> bool,
        err: impl FnOnce() -> Err,
    ) -> Self::Output {
        self.filter(pred).ok_or_else(err)
    }
}

impl<Val, Fun, Out> Apply<Fun> for Option<Val>
where
    bool: Combine<Out>,
    Fun: Fn(Val) -> Out,
    Out: Response,
{
    type Output = <bool as Combine<Out>>::Output;
    fn apply(self, f: &Fun) -> Self::Output {
        self.is_some().combine(|| f(self.unwrap()))
    }
}

impl<Val> Combine<()> for Option<Val> {
    type Output = Self;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(),
    {
        let value = self?;
        f();
        Some(value)
    }
}

impl<Val> Combine<bool> for Option<Val> {
    type Output = Option<Val>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> bool,
    {
        match self {
            Some(value) => match f() {
                true => Some(value),
                false => None,
            },
            None => None,
        }
    }
}

impl<Val0, Val1> Combine<Option<Val1>> for Option<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Option<Val1>,
    {
        self.and_then(|val0| f().map(|val1| (val0, val1)))
    }
}

impl<Val0, Val1> Combine<Sure<Val1>> for Option<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Sure<Val1>,
    {
        Some((self?, f().value()))
    }
}

impl<Val> Switch<()> for Option<Val> {
    type Output = Sure<Self>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> (),
    {
        match self {
            Some(value) => Sure(Some(value)),
            None => {
                f();
                Sure(None)
            }
        }
    }
}

impl<Val> Switch<bool> for Option<Val> {
    type Output = Option<Option<Val>>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> bool,
    {
        match self {
            Some(value) => Some(Some(value)),
            None => f().then_some(None),
        }
    }
}

impl<Val> Switch<Option<Val>> for Option<Val> {
    type Output = Option<Val>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Option<Val>,
    {
        match self {
            Some(value) => Some(value),
            None => f(),
        }
    }
}

impl<Val> Switch<Sure<Val>> for Option<Val> {
    type Output = Sure<Val>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Sure<Val>,
    {
        match self {
            Some(value) => Sure(value),
            None => f(),
        }
    }
}

impl<Val, Err> Switch<Result<Val, Err>> for Option<Val> {
    type Output = Result<Val, Err>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Result<Val, Err>,
    {
        match self {
            Some(value) => Ok(value),
            None => f(),
        }
    }
}

impl<Val, Err> Switch<Unsure<Err>> for Option<Val> {
    type Output = Result<Option<Val>, Err>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Unsure<Err>,
    {
        match self {
            Some(value) => Ok(Some(value)),
            None => f().into_result().map(|()| None),
        }
    }
}
