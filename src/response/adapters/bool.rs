use crate::response::prelude::internal::*;

impl Response for bool {
    type Value = ();
    type Error = ();
    type Residual = Exception<()>;
    type WithVal<Val> = bool;
    type WithErr<Err> = bool;

    fn from_value((): Self::Value) -> Self {
        true
    }

    fn from_error((): Self::Error) -> Self {
        false
    }

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        match self {
            true => ControlFlow::Continue(()),
            false => ControlFlow::Break(()),
        }
    }

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        match self {
            true => {
                f(());
                self
            }
            false => self,
        }
    }

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
        match self {
            true => true,
            false => {
                f(());
                false
            }
        }
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        f(())
    }
}

impl<Fun, Val> Select<Fun> for bool
where
    Fun: Fn() -> Val,
{
    type Output = Option<Val>;

    fn sel(self, f: &Fun) -> Self::Output {
        match self {
            true => Some(f()),
            false => None,
        }
    }
}

impl Attach for bool {
    type Output<V> = Option<V>;

    fn attach_to_response<V>(self, value: impl FnOnce() -> V) -> Self::Output<V> {
        self.then(value)
    }
}

impl AttachErr for bool {
    type Output<E> = Unsure<E>;

    fn attach_err_to_response<E>(self, value: impl FnOnce() -> E) -> Self::Output<E> {
        match self {
            true => Unsure::ok(),
            false => Unsure::err(value()),
        }
    }
}

impl<Fun, Err> SelectErr<Fun> for bool
where
    Fun: Fn() -> Err,
{
    type Output = Unsure<Err>;

    fn sel_err(self, f: &Fun) -> Self::Output {
        match self {
            true => Unsure::ok(),
            false => Unsure::err(f()),
        }
    }
}

impl Fallible for bool {
    type Infallible = ();
    type Optional = Sure<bool>;

    fn optional(self) -> Self::Optional {
        Sure(self)
    }
}

impl<Fun, Out> Apply<Fun> for bool
where
    Self: Combine<Out>,
    Fun: Fn() -> Out,
    Out: Response,
{
    type Output = <Self as Combine<Out>>::Output;

    fn apply(self, f: &Fun) -> Self::Output {
        self.combine(f)
    }
}

impl Combine<()> for bool {
    type Output = Self;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(),
    {
        match self {
            true => {
                f();
                true
            }
            false => false,
        }
    }
}

impl Combine<bool> for bool {
    type Output = Self;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> bool,
    {
        self && f()
    }
}

impl<Val> Combine<Sure<Val>> for bool {
    type Output = Option<Val>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Sure<Val>,
    {
        match self {
            true => Some(f().value()),
            false => None,
        }
    }
}

impl<Val> Combine<Option<Val>> for bool {
    type Output = Option<Val>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Option<Val>,
    {
        match self {
            true => f(),
            false => None,
        }
    }
}

impl<Val, Err> Combine<Result<Val, Err>> for bool {
    type Output = Option<Val>;

    fn combine<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Result<Val, Err>,
    {
        match self {
            true => f().ok(),
            false => None,
        }
    }
}

impl Switch<()> for bool {
    type Output = ();

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> (),
    {
        match self {
            true => (),
            false => f(),
        }
    }
}

impl Switch<bool> for bool {
    type Output = Self;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> bool,
    {
        match self {
            true => true,
            false => f(),
        }
    }
}

impl<Val> Switch<Sure<Val>> for bool {
    type Output = Sure<Option<Val>>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Sure<Val>,
    {
        match self {
            true => Sure(None),
            false => f().map(Some),
        }
    }
}

impl<Val> Switch<Option<Val>> for bool {
    type Output = Option<Option<Val>>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Option<Val>,
    {
        match self {
            true => Some(None),
            false => f().map(Some),
        }
    }
}

impl<Val, Err> Switch<Result<Val, Err>> for bool {
    type Output = Result<Option<Val>, Err>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Result<Val, Err>,
    {
        match self {
            true => Ok(None),
            false => f().map(Some),
        }
    }
}

impl<Err> Switch<Unsure<Err>> for bool {
    type Output = Unsure<Err>;

    fn switch<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Unsure<Err>,
    {
        match self {
            true => Unsure::ok(),
            false => f(),
        }
    }
}
