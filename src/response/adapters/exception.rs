use crate::response::prelude::*;

pub struct Exception<T>(pub T);

impl<T> Exception<T> {
    pub fn error(self) -> T {
        self.0
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Response for Exception<T> {
    type Value = Infallible;
    type Error = T;
    type Residual = Self;
    type WithVal<Val> = Exception<T>;
    type WithErr<Err> = Exception<Err>;

    fn from_value(_value: Self::Value) -> Self {
        unreachable!()
    }

    fn from_error(error: Self::Error) -> Self {
        Exception(error)
    }

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        ControlFlow::Break(self.error())
    }

    fn map<Fun, Val>(self, _: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        self
    }

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
        Exception(f(self.error()))
    }

    fn flat_map<Fun, Val>(self, _f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        self
    }
}

impl<T> ErrorResponse for Exception<T> {
    type VoidErr = bool;

    fn void_err(self) -> Self::VoidErr {
        false
    }

    fn unwrap_err(self) -> Self::Error {
        self.error()
    }
}

impl<Val> Combine<()> for Exception<Val> {
    type Output = Self;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(),
    {
        f();
        self
    }
}

impl<Val> Combine<bool> for Exception<Val> {
    type Output = Option<Val>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> bool,
    {
        match f() {
            true => Some(self.error()),
            false => None,
        }
    }
}

impl<Val0, Val1> Combine<Exception<Val1>> for Exception<Val0> {
    type Output = Exception<(Val0, Val1)>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Exception<Val1>,
    {
        Exception((self.error(), f().error()))
    }
}

impl<Val0, Val1> Combine<Option<Val1>> for Exception<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Option<Val1>,
    {
        Some((self.error(), f()?))
    }
}

impl<Val, Err> Combine<Unsure<Err>> for Exception<Val> {
    type Output = Result<Val, Err>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Unsure<Err>,
    {
        f().into_result()?;
        Ok(self.error())
    }
}

impl<Val0, Val1, Err> Combine<Result<Val1, Err>> for Exception<Val0> {
    type Output = Result<(Val0, Val1), Err>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Result<Val1, Err>,
    {
        let value = f()?;
        Ok((self.error(), value))
    }
}

impl<I, O> FromErr<O> for Exception<I>
where
    I: From<O>,
{
    fn from_err(v: O) -> Self {
        Exception(v.into())
    }
}

impl<I, O> IntoErr<O> for Exception<I>
where
    I: Into<O>,
{
    fn into_err(self) -> O {
        self.error().into()
    }
}
