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

impl<T> Response for Unsure<T> {
    type Value = ();
    type Error = T;
    type WithVal<Val> = Unsure<T>;
    type WithErr<Err> = Unsure<Err>;

    fn from_value((): Self::Value) -> Self {
        Unsure::ok()
    }

    fn from_error(error: Self::Error) -> Self {
        Unsure::err(error)
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

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        self.into_result().control_flow()
    }
}

impl<Fun, Val, Err> Mappable<Fun> for Unsure<Err>
where
    Fun: Fn() -> Val,
{
    type Output = Result<Val, Err>;

    fn map_response(self, f: &Fun) -> Self::Output {
        self.into_result().map(|()| f())
    }
}

impl<Err> Combinable<()> for Unsure<Err> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce(),
    {
        match self.into_result() {
            Ok(()) => {
                response();
                Unsure::ok()
            }
            Err(error) => Unsure::err(error),
        }
    }
}

impl<Err, Val> Combinable<Sure<Val>> for Unsure<Err> {
    type Output = Result<Val, Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Sure<Val>,
    {
        self.into_result()?;
        Ok(response().value())
    }
}

impl<Err> Combinable<Unsure<Err>> for Unsure<Err> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Unsure<Err>,
    {
        match self.into_result() {
            Ok(()) => response(),
            Err(error) => Self::err(error),
        }
    }
}

impl<Err, Val> Combinable<Result<Val, Err>> for Unsure<Err> {
    type Output = Result<Val, Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Result<Val, Err>,
    {
        self.into_result()?;
        response()
    }
}

impl<Err> Disjoinable<Unsure<Err>> for Unsure<Err> {
    type Output = Unsure<(Err, Err)>;

    fn disjoin_response<Fun, Rec, Str>(
        self,
        response: Fun,
        recover: Rec,
        stream: &mut Str,
    ) -> Self::Output
    where
        Fun: FnOnce(&mut Str) -> Unsure<Err>,
        Rec: FnOnce(&mut Str),
    {
        match self.into_result() {
            Ok(()) => Unsure::ok(),
            Err(error0) => {
                recover(stream);
                response(stream).map_err(|error1| (error0, error1))
            }
        }
    }
}

impl<Err> Attachable for Unsure<Err> {
    type Output<V> = Result<V, Err>;

    fn attach_to_response<V>(self, value: V) -> Self::Output<V> {
        self.into_result().map(|()| value)
    }
}

impl<Err> Recoverable for Unsure<Err> {
    fn recover_response<Rec, Str>(self, on_residual: Rec, stream: &mut Str) -> Self
    where
        Rec: FnOnce(&mut Str),
    {
        match self.into_result() {
            Ok(()) => Unsure::ok(),
            Err(error) => {
                on_residual(stream);
                Unsure::err(error)
            }
        }
    }
}

impl<Err> Optionable for Unsure<Err> {
    type Output = ();

    fn opt_response(self) -> Self::Output {
        let _ = self;
    }
}

impl<Err> Fallible for Unsure<Err> {
    type Infallible = ();
}
