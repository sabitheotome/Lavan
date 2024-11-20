use crate::response::prelude::*;

pub struct Unsure<E>(pub Result<(), E>);

impl<E> Unsure<E> {
    pub fn ok() -> Self {
        Self(Ok(()))
    }

    pub fn err(error: E) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<(), E> {
        self.0
    }

    pub fn unwrap(self)
    where
        E: std::fmt::Debug,
    {
        self.into_result().unwrap();
    }

    pub fn unwrap_err(self) -> E
    where
        E: std::fmt::Debug,
    {
        self.into_result().unwrap_err()
    }
}

impl<E> From<Result<(), E>> for Unsure<E> {
    fn from(value: Result<(), E>) -> Self {
        Self(value)
    }
}

impl<E> From<Unsure<E>> for Result<(), E> {
    fn from(value: Unsure<E>) -> Self {
        value.0
    }
}

impl<E> From<Option<E>> for Unsure<E> {
    fn from(value: Option<E>) -> Self {
        match value {
            Some(err) => Unsure::err(err),
            None => Unsure::ok(),
        }
    }
}

impl<E> From<Unsure<E>> for Option<E> {
    fn from(value: Unsure<E>) -> Self {
        value.0.err()
    }
}

impl<T> Response for Unsure<T> {
    type Value = ();
    type Error = T;
    type Residual = Exception<T>;
    type WithVal<Val> = Unsure<T>;
    type WithErr<Err> = Unsure<Err>;

    fn from_value((): Self::Value) -> Self {
        Unsure::ok()
    }

    fn from_error(error: Self::Error) -> Self {
        Unsure::err(error)
    }

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        self.into_result().control_flow()
    }

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        self.into_result()
            .map(|()| {
                f(());
            })
            .into()
    }

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
        self.into_result().map_err(f).into()
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        f(())
    }
}

impl<Err> ErrorResponse for Unsure<Err> {
    type VoidErr = bool;

    fn void_err(self) -> Self::VoidErr {
        self.into_result().is_ok()
    }

    fn unwrap_err(self) -> Self::Error
    where
        Self::Value: std::fmt::Debug,
    {
        self.into_result().unwrap_err()
    }
}

impl<Fun, Val, Err> Select<Fun> for Unsure<Err>
where
    Fun: Fn() -> Val,
{
    type Output = Result<Val, Err>;

    fn sel(self, f: &Fun) -> Self::Output {
        self.into_result().map(|()| f())
    }
}

impl<Err> Attach for Unsure<Err> {
    type Output<V> = Result<V, Err>;

    fn attach_to_response<V>(self, value: impl FnOnce() -> V) -> Self::Output<V> {
        self.into_result().map(|()| value())
    }
}

impl<Err> Fallible for Unsure<Err> {
    type Infallible = ();
    type Optional = Sure<bool>;

    fn optional(self) -> Self::Optional {
        Sure(self.into_result().is_ok())
    }
}

impl<Err, Fun, Out> Apply<Fun> for Unsure<Err>
where
    Result<(), Err>: Combine<Out>,
    Fun: Fn() -> Out,
    Out: Response,
{
    type Output = <Result<(), Err> as Combine<Out>>::Output;

    fn apply(self, f: &Fun) -> Self::Output {
        self.into_result().combine(|| f())
    }
}

impl<Err> Combine<()> for Unsure<Err> {
    type Output = Self;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(),
    {
        match self.into_result() {
            Ok(()) => {
                f();
                Unsure::ok()
            }
            Err(error) => Unsure::err(error),
        }
    }
}

impl<Err, Val> Combine<Sure<Val>> for Unsure<Err> {
    type Output = Result<Val, Err>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Sure<Val>,
    {
        self.into_result()?;
        Ok(f().value())
    }
}

impl<Err> Combine<Unsure<Err>> for Unsure<Err> {
    type Output = Self;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Unsure<Err>,
    {
        match self.into_result() {
            Ok(()) => f(),
            Err(error) => Self::err(error),
        }
    }
}

impl<Err, Val> Combine<Result<Val, Err>> for Unsure<Err> {
    type Output = Result<Val, Err>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Result<Val, Err>,
    {
        self.into_result()?;
        f()
    }
}

impl<Err> Switch<()> for Unsure<Err> {
    type Output = ();

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> (),
    {
        match self.into_result() {
            Ok(()) => {}
            Err(_error) => {
                f();
            }
        }
    }
}

impl<Err> Switch<bool> for Unsure<Err> {
    type Output = Unsure<Err>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> bool,
    {
        match self.into_result() {
            Ok(()) => Unsure::ok(),
            Err(error) => f().then_some(()).ok_or(error).into(),
        }
    }
}

impl<Val, Err> Switch<Option<Val>> for Unsure<Err> {
    type Output = Result<Option<Val>, Err>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Option<Val>,
    {
        match self.into_result() {
            Ok(()) => Ok(None),
            Err(error) => f().map(Some).ok_or(error),
        }
    }
}

impl<Val, Err> Switch<Sure<Val>> for Unsure<Err> {
    type Output = Sure<Option<Val>>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Sure<Val>,
    {
        match self.into_result() {
            Ok(()) => Sure(None),
            Err(_error) => f().map(Some),
        }
    }
}

impl<Val, Err0, Err1> Switch<Result<Val, Err1>> for Unsure<Err0> {
    type Output = Result<Option<Val>, (Err0, Err1)>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Result<Val, Err1>,
    {
        match self.into_result() {
            Ok(()) => Ok(None),
            Err(error0) => f().map(Some).map_err(|error1| (error0, error1)),
        }
    }
}

impl<Err0, Err1> Switch<Unsure<Err1>> for Unsure<Err0> {
    type Output = Unsure<(Err0, Err1)>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Unsure<Err1>,
    {
        match self.into_result() {
            Ok(()) => Unsure::ok(),
            Err(error0) => f().map_err(|error1| (error0, error1)),
        }
    }
}
