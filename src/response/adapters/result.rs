use crate::response::prelude::*;

impl<T, E> Response for Result<T, E> {
    type Value = T;
    type WithVal<Val> = Result<Val, E>;
    type Error = E;
    type WithErr<Err> = Result<T, Err>;

    fn from_value(value: Self::Value) -> Self {
        Ok(value)
    }

    fn from_error(error: Self::Error) -> Self {
        Err(error)
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

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        match self {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(e),
        }
    }
}

impl<T, E> ValueFunctor for Result<T, E> {
    fn unwrap(self) -> Self::Value
    where
        Self::Error: std::fmt::Debug,
    {
        self.unwrap()
    }
}

impl<T, E> ErrorFunctor for Result<T, E> {
    fn unwrap_err(self) -> Self::Error
    where
        Self::Value: std::fmt::Debug,
    {
        self.unwrap_err()
    }
}

impl<Val, Err> Combinable<()> for Result<Val, Err> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce(),
    {
        let value = self?;
        response();
        Ok(value)
    }
}

impl<Val0, Val1, Err0, Err1> Combinable<Result<Val1, Err1>> for Result<Val0, Err0>
where
    Err1: From<Err0>,
{
    type Output = Result<(Val0, Val1), Err1>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Result<Val1, Err1>,
    {
        Ok((self?, response()?))
    }
}

impl<Val0, Val1, Err> Combinable<Sure<Val1>> for Result<Val0, Err> {
    type Output = Result<(Val0, Val1), Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Sure<Val1>,
    {
        let value = self?;
        Ok((value, response().value()))
    }
}

impl<Val, Err> Combinable<Unsure<Err>> for Result<Val, Err> {
    type Output = Result<Val, Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Unsure<Err>,
    {
        let value = self?;
        response().into_result()?;
        Ok(value)
    }
}

impl<Val, Err0, Err1> Switchable<Result<Val, Err1>> for Result<Val, Err0> {
    type Output = Result<Val, (Err0, Err1)>;

    fn disjoin_response<Fun, Rec, Str>(
        self,
        response: Fun,
        recover: Rec,
        stream: &mut Str,
    ) -> Self::Output
    where
        Fun: FnOnce(&mut Str) -> Result<Val, Err1>,
        Rec: FnOnce(&mut Str),
    {
        match self {
            Ok(value) => Ok(value),
            Err(error0) => {
                recover(stream);
                match response(stream) {
                    Ok(value) => Ok(value),
                    Err(error1) => Err((error0, error1)),
                }
            }
        }
    }
}

impl<Val, Err> Recoverable for Result<Val, Err> {
    fn recover_response<Rec, Str>(self, on_residual: Rec, stream: &mut Str) -> Self
    where
        Rec: FnOnce(&mut Str),
    {
        match self {
            Ok(value) => Ok(value),
            Err(error) => {
                on_residual(stream);
                Err(error)
            }
        }
    }
}

impl<Val, Err> Ignorable for Result<Val, Err> {
    type Output = Unsure<Err>;

    fn ignore_response(self) -> Self::Output {
        self.map(|_| ()).into()
    }
}

impl<Val, Err> Optionable for Result<Val, Err> {
    type Output = Sure<Option<Val>>;

    fn opt_response(self) -> Self::Output {
        Sure(self.ok())
    }
}

impl<Val, Err> Fallible for Result<Val, Err> {
    type Infallible = Sure<Self::Value>;
}

impl<Val, Err> FilterableWithErr<Err> for Result<Val, Err> {
    type Output = Result<Val, Err>;

    fn filter_response_or_else(
        self,
        predicate: impl FnOnce(&Self::Value) -> bool,
        error: impl FnOnce() -> Err,
    ) -> Self::Output {
        match self {
            Ok(ok) => match predicate(&ok) {
                true => Ok(ok),
                false => Err(error()),
            },
            Err(err) => Err(err),
        }
    }
}

impl<Val, Err, Fun, Out> Bindable<Fun> for Result<Val, Err>
where
    Unsure<Err>: Combinable<Out>,
    Fun: Fn(Val) -> Out,
    Out: Response,
{
    type Output = <Unsure<Err> as Combinable<Out>>::Output;
    fn bind(self, f: &Fun) -> Self::Output {
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
        unsure.combine_response(|| f(option.unwrap()))
    }
}
