use crate::response::prelude::*;

impl Response for () {}

impl Pseudodata for () {
    type Value = ();
    type WithVal<Val> = ();

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        f(());
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        f(());
    }
}

impl Pure for () {
    type Value = ();
    fn pure(value: Self::Value) -> Self {}
    fn unwrap(self) -> Self::Value {}
}

impl<Fun, Val> Mappable<Fun> for ()
where
    Fun: Fn() -> Val,
{
    type Output = Sure<Val>;

    fn map_response(self, f: &Fun) -> Self::Output {
        Sure(f())
    }
}

impl<Res> Combinable<Res> for ()
where
    Res: Response,
{
    type Output = Res;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Res,
    {
        response()
    }
}

impl Pseudotriable for () {
    type Output = ();
    type Residual = Infallible;

    fn from_output((): Self::Output) -> Self {}

    fn from_residual(_error: Self::Residual) -> Self {
        unreachable!()
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        ControlFlow::Continue(())
    }
}
