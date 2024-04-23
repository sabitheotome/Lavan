use crate::response::prelude::*;

impl<T> Response for Option<T> {
    type Value = T;
    type Error = ();
    type WithVal<Val> = Option<Val>;
    type WithErr<Err> = Option<T>;
    fn from_value(value: Self::Value) -> Self {
        Some(value)
    }

    fn from_error((): Self::Error) -> Self {
        None
    }

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        self.map(f)
    }

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
        match self {
            Some(val) => Some(val),
            None => {
                f(());
                None
            }
        }
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        self.and_then(f)
    }

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        match self {
            Some(v) => ControlFlow::Continue(v),
            None => ControlFlow::Break(()),
        }
    }
}

impl<T> ValueFunctor for Option<T> {
    fn unwrap(self) -> Self::Value {
        self.unwrap()
    }
}

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

impl<Val> Switchable<Option<Val>> for Option<Val> {
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

impl<Fun, Val, Err> ErrMappable<Fun> for Option<Val>
where
    Fun: Fn() -> Err,
{
    type Output = Result<Val, Err>;

    fn err_map_response(self, f: &Fun) -> Self::Output {
        match self {
            Some(value) => Ok(value),
            None => Err(f()),
        }
    }
}

impl<Val> Optionable for Option<Val> {
    type Output = Sure<Option<Val>>;

    fn opt_response(self) -> Self::Output {
        Sure(self)
    }
}

impl<Val> Fallible for Option<Val> {
    type Infallible = Sure<Self::Value>;
}

impl<Val> Filterable for Option<Val> {
    type Output = Option<Val>;

    fn filter_response(self, predicate: impl FnOnce(&Self::Value) -> bool) -> Self::Output {
        self.filter(predicate)
    }
}

impl<Val, Err> FilterableWithErr<Err> for Option<Val> {
    type Output = Result<Val, Err>;

    fn filter_response_or_else(
        self,
        predicate: impl FnOnce(&Self::Value) -> bool,
        error: impl FnOnce() -> Err,
    ) -> Self::Output {
        self.filter(predicate).ok_or_else(error)
    }
}

impl<Val, Fun, Out> Bindable<Fun> for Option<Val>
where
    bool: Combinable<Out>,
    Fun: Fn(Val) -> Out,
    Out: Response,
{
    type Output = <bool as Combinable<Out>>::Output;
    fn bind(self, f: &Fun) -> Self::Output {
        self.is_some().combine_response(|| f(self.unwrap()))
    }
}
