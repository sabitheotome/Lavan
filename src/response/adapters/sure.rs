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
}

impl<T> Response for Sure<T> {}

impl<T> Pseudodata for Sure<T> {
    type Value = T;
    type WithVal<Val> = Sure<Val>;

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        Sure(f(self.value()))
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        f(self.value())
    }
}

impl<T> Data for Sure<T> {}

impl<T> Pure for Sure<T> {
    type Value = T;

    fn pure(value: Self::Value) -> Self {
        Sure(value)
    }

    fn unwrap(self) -> Self::Value {
        self.value()
    }
}

impl<Val> Combinable<()> for Sure<Val> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce(),
    {
        response();
        self
    }
}

impl<Val> Combinable<bool> for Sure<Val> {
    type Output = Option<Val>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> bool,
    {
        match response() {
            true => Some(self.value()),
            false => None,
        }
    }
}

impl<Val0, Val1> Combinable<Sure<Val1>> for Sure<Val0> {
    type Output = Sure<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Sure<Val1>,
    {
        Sure((self.value(), response().value()))
    }
}

impl<Val0, Val1> Combinable<Option<Val1>> for Sure<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Option<Val1>,
    {
        Some((self.value(), response()?))
    }
}

impl<Val, Err> Combinable<Unsure<Err>> for Sure<Val> {
    type Output = Result<Val, Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Unsure<Err>,
    {
        response().into_result()?;
        Ok(self.value())
    }
}

impl<Val0, Val1, Err> Combinable<Result<Val1, Err>> for Sure<Val0> {
    type Output = Result<(Val0, Val1), Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Result<Val1, Err>,
    {
        let value = response()?;
        Ok((self.value(), value))
    }
}

impl<Val> Ignorable for Sure<Val> {
    type Output = ();

    fn ignore_response(self) -> Self::Output {
        let _ = self;
    }
}

impl<Val> Pseudotriable for Sure<Val> {
    type Output = Val;
    type Residual = Infallible;

    fn from_output(value: Self::Output) -> Self {
        Sure(value)
    }

    fn from_residual(_error: Self::Residual) -> Self {
        unreachable!()
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        ControlFlow::Continue(self.value())
    }
}
