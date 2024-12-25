use crate::response::prelude::internal::*;

impl Response for () {
    type Value = ();
    type Error = Infallible;
    type Residual = Infallible;
    type WithVal<Val> = ();
    type WithErr<Err> = ();

    fn from_value((): Self::Value) -> Self {}

    fn from_error(_: Self::Error) -> Self {
        unreachable!()
    }

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        ControlFlow::Continue(())
    }

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        f(());
    }

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        f(());
    }
}

impl<Fun, Val> Select<Fun> for ()
where
    Fun: Fn() -> Val,
{
    type Output = Sure<Val>;

    fn sel(self, f: &Fun) -> Self::Output {
        Sure(f())
    }
}

impl Attach for () {
    type Output<V> = Sure<V>;

    fn attach_to_response<V>(self, value: impl FnOnce() -> V) -> Self::Output<V> {
        Sure(value())
    }
}

impl<Fun, Out> Apply<Fun> for ()
where
    Fun: Fn() -> Out,
    Out: Response,
{
    type Output = Out;
    fn apply(self, f: &Fun) -> Self::Output {
        f()
    }
}

impl<Res> Combine<Res> for ()
where
    Res: Response,
{
    type Output = Res;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Res,
    {
        f()
    }
}

impl<O> IntoErr<O> for Infallible {
    fn into_err(self) -> O {
        match self {}
    }
}

impl<O> FromErr<O> for Infallible {
    fn from_err(v: O) -> Self {
        let _ = v;
        unreachable!()
    }
}
