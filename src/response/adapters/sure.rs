use crate::response::prelude::*;

pub struct Sure<T>(pub T);

impl<T> Sure<T> {
    pub fn value(self) -> T {
        self.0
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn map<Fun, Val>(self, f: Fun) -> Sure<Val>
    where
        Fun: FnOnce(T) -> Val,
    {
        Sure(f(self.value()))
    }
}

impl<T> Response for Sure<T> {
    type Value = T;
    type Error = Infallible;
    type Residual = Infallible;
    type WithVal<Val> = Sure<Val>;
    type WithErr<Err> = Sure<T>;

    fn from_value(value: Self::Value) -> Self {
        Sure(value)
    }

    fn from_error(_error: Self::Error) -> Self {
        unreachable!()
    }

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        ControlFlow::Continue(self.value())
    }

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        self.map(f)
    }

    fn map_err<Fun, Err>(self, _: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
        self
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        f(self.value())
    }
}

impl<T> IntoIterator for Sure<T> {
    type Item = T;
    type IntoIter = std::option::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self.value()).into_iter()
    }
}

impl<T> ValueResponse for Sure<T> {
    type VoidVal = ();

    fn void_val(self) -> Self::VoidVal {
        let _ = self.0;
    }

    fn unwrap(self) -> Self::Value {
        self.value()
    }
}

impl<Val> Predict for Sure<Val> {
    type Output = Option<Val>;

    fn predict(self, pred: impl FnOnce(&Self::Value) -> bool) -> Self::Output {
        match pred(self.get()) {
            true => Some(self.value()),
            false => None,
        }
    }
}

impl<Val, Err> PredictOrElse<Err> for Sure<Val> {
    type Output = Result<Val, Err>;

    fn predict_or_else(
        self,
        pred: impl FnOnce(&Self::Value) -> bool,
        err: impl FnOnce() -> Err,
    ) -> Self::Output {
        match pred(self.get()) {
            true => Ok(self.value()),
            false => Err(err()),
        }
    }
}

impl<Val, Fun, Out> Apply<Fun> for Sure<Val>
where
    Fun: Fn(Val) -> Out,
    Out: Response,
{
    type Output = Out;

    fn apply(self, f: &Fun) -> Self::Output {
        f(self.value())
    }
}

impl<Val> Combine<()> for Sure<Val> {
    type Output = Self;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(),
    {
        f();
        self
    }
}

impl<Val> Combine<bool> for Sure<Val> {
    type Output = Option<Val>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> bool,
    {
        match f() {
            true => Some(self.value()),
            false => None,
        }
    }
}

impl<Val0, Val1> Combine<Sure<Val1>> for Sure<Val0> {
    type Output = Sure<(Val0, Val1)>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Sure<Val1>,
    {
        Sure((self.value(), f().value()))
    }
}

impl<Val0, Val1> Combine<Option<Val1>> for Sure<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Option<Val1>,
    {
        Some((self.value(), f()?))
    }
}

impl<Val, Err> Combine<Unsure<Err>> for Sure<Val> {
    type Output = Result<Val, Err>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Unsure<Err>,
    {
        f().into_result()?;
        Ok(self.value())
    }
}

impl<Val0, Val1, Err> Combine<Result<Val1, Err>> for Sure<Val0> {
    type Output = Result<(Val0, Val1), Err>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Result<Val1, Err>,
    {
        let value = f()?;
        Ok((self.value(), value))
    }
}
