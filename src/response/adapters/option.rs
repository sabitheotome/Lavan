use crate::response::prelude::*;

impl<T> Response for Option<T> {}

impl<T> Pseudodata for Option<T> {
    type Value = T;
    type WithVal<Val> = Option<Val>;

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

impl<T> Data for Option<T> {}

impl<Val> Combinable<()> for Option<Val> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce(),
    {
        let value = self?;
        response();
        Some(value)
    }
}

impl<Val> Combinable<bool> for Option<Val> {
    type Output = Option<Val>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> bool,
    {
        match self {
            Some(value) => match response() {
                true => Some(value),
                false => None,
            },
            None => None,
        }
    }
}

impl<Val0, Val1> Combinable<Option<Val1>> for Option<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Option<Val1>,
    {
        self.and_then(|val0| response().map(|val1| (val0, val1)))
    }
}

impl<Val0, Val1> Combinable<Sure<Val1>> for Option<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Sure<Val1>,
    {
        Some((self?, response().value()))
    }
}

impl<Val> Disjoinable<Option<Val>> for Option<Val> {
    type Output = Option<Val>;

    fn disjoin_response<Fun, Rec, Str>(
        self,
        response: Fun,
        recover: Rec,
        stream: &mut Str,
    ) -> Self::Output
    where
        Fun: FnOnce(&mut Str) -> Option<Val>,
        Rec: FnOnce(&mut Str),
    {
        match self {
            Some(value) => Some(value),
            None => {
                recover(stream);
                response(stream)
            }
        }
    }
}

impl<Val> Recoverable for Option<Val> {
    fn recover_response<Rec, Str>(self, on_residual: Rec, stream: &mut Str) -> Self
    where
        Rec: FnOnce(&mut Str),
    {
        match self {
            Some(value) => Some(value),
            None => {
                on_residual(stream);
                None
            }
        }
    }
}

impl<Val> Ignorable for Option<Val> {
    type Output = bool;

    fn ignore_response(self) -> Self::Output {
        self.is_some()
    }
}

impl<Val> Optionable for Option<Val> {
    type Output = Sure<Option<Val>>;

    fn opt_response(self) -> Self::Output {
        Sure(self)
    }
}

impl<Val> Pseudotriable for Option<Val> {
    type Output = Val;
    type Residual = ();

    fn from_output(value: Self::Output) -> Self {
        Some(value)
    }

    fn from_residual((): Self::Residual) -> Self {
        None
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Some(v) => ControlFlow::Continue(v),
            None => ControlFlow::Break(()),
        }
    }
}

impl<Val> Triable for Option<Val> {
    type Infallible = Sure<Self::Output>;
}
