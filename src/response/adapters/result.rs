use crate::response::prelude::*;

impl<T, E> Response for Result<T, E> {}

impl<T, E> Pseudodata for Result<T, E> {
    type Value = T;
    type WithVal<Val> = Result<Val, E>;

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

impl<T, E> Data for Result<T, E> {}

impl<T, E> Exceptional for Result<T, E> {
    type Error = E;
    type WithErr<Err> = Result<T, Err>;

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
        self.map_err(f)
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

impl<Val0, Val1, Err> Combinable<Result<Val1, Err>> for Result<Val0, Err> {
    type Output = Result<(Val0, Val1), Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Result<Val1, Err>,
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

impl<Val, Err0, Err1> Disjoinable<Result<Val, Err1>> for Result<Val, Err0> {
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

impl<Val, Err> Pseudotriable for Result<Val, Err> {
    type Output = Val;
    type Residual = Err;

    fn from_output(value: Self::Output) -> Self {
        Ok(value)
    }

    fn from_residual(error: Self::Residual) -> Self {
        Err(error)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(e),
        }
    }
}

impl<Val, Err> Triable for Result<Val, Err> {
    type Infallible = Sure<Self::Output>;
}
