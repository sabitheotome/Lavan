use crate::output::prelude::*;

impl<T> Response for Option<T> {
    type Value = T;
    type Error = ();
    type WithVal<Val> = Option<Val>;
    type WithErr<Err> = Option<T>;
    fn from_value(value: Self::Value) -> Self {
        Some(value)
    }

    fn from_error((): Self::Error) -> Self {
        None
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

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        match self {
            Some(v) => ControlFlow::Continue(v),
            None => ControlFlow::Break(()),
        }
    }
}

impl<T> ValueFunctor for Option<T> {
    fn unwrap(self) -> Self::Value {
        self.unwrap()
    }
}

impl<Val> Ignorable for Option<Val> {
    type Output = bool;

    fn ignore_response(self) -> Self::Output {
        self.is_some()
    }
}

impl<Fun, Val, Err> ErrMappable<Fun> for Option<Val>
where
    Fun: Fn() -> Err,
{
    type Output = Result<Val, Err>;

    fn err_map_response(self, f: &Fun) -> Self::Output {
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

impl<Val> Filterable for Option<Val> {
    type Output = Option<Val>;

    fn filter_response(self, predicate: impl FnOnce(&Self::Value) -> bool) -> Self::Output {
        self.filter(predicate)
    }
}

impl<Val, Err> FilterableWithErr<Err> for Option<Val> {
    type Output = Result<Val, Err>;

    fn filter_response_or_else(
        self,
        predicate: impl FnOnce(&Self::Value) -> bool,
        error: impl FnOnce() -> Err,
    ) -> Self::Output {
        self.filter(predicate).ok_or_else(error)
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
