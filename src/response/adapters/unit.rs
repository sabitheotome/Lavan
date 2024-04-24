use crate::response::prelude::*;

impl Response for () {
    type Value = ();
    type Error = Infallible;
    type WithVal<Val> = ();
    type WithErr<Err> = ();

    fn from_value((): Self::Value) -> Self {}

    fn from_error(_: Self::Error) -> Self {
        unreachable!()
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

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        ControlFlow::Continue(())
    }
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

impl Attachable for () {
    type Output<V> = Sure<V>;

    fn attach_to_response<V>(self, value: impl FnOnce() -> V) -> Self::Output<V> {
        Sure(value())
    }
}

impl<Fun, Out> Bindable<Fun> for ()
where
    Fun: Fn() -> Out,
    Out: Response,
{
    type Output = Out;
    fn bind(self, f: &Fun) -> Self::Output {
        f()
    }
}
