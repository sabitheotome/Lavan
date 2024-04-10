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

impl<T> Response for Sure<T> {
    type Value = T;
    type Error = Infallible;
    type WithVal<Val> = Sure<Val>;
    type WithErr<Err> = Sure<T>;

    fn from_value(value: Self::Value) -> Self {
        Sure(value)
    }

    fn from_error(_error: Self::Error) -> Self {
        unreachable!()
    }

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        Sure(f(self.value()))
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

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        ControlFlow::Continue(self.value())
    }
}

impl<T> ValueFunctor for Sure<T> {}

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

impl<Val> Filterable for Sure<Val> {
    type Output = Option<Val>;

    fn filter_response(self, predicate: impl FnOnce(&Self::Value) -> bool) -> Self::Output {
        match predicate(self.get()) {
            true => Some(self.value()),
            false => None,
        }
    }
}

impl<Val, Err> FilterableWithErr<Err> for Sure<Val> {
    type Output = Result<Val, Err>;

    fn filter_response_or_else(
        self,
        predicate: impl FnOnce(&Self::Value) -> bool,
        error: impl FnOnce() -> Err,
    ) -> Self::Output {
        match predicate(self.get()) {
            true => Ok(self.value()),
            false => Err(error()),
        }
    }
}

impl<Val, Fun, Out> Bindable<Fun> for Sure<Val>
where
    Fun: Fn(Val) -> Out,
    Out: Response,
{
    type Output = Out;
    fn bind(self, f: &Fun) -> Self::Output {
        f(self.value())
    }
}
