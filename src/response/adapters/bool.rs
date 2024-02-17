use crate::response::prelude::*;

impl Response for bool {
    type Value = ();
    type Error = ();
    type WithVal<Val> = bool;
    type WithErr<Err> = bool;

    fn from_value((): Self::Value) -> Self {
        true
    }

    fn from_error((): Self::Error) -> Self {
        false
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

    fn control_flow(self) -> ControlFlow<Self::Error, Self::Value> {
        match self {
            true => ControlFlow::Continue(()),
            false => ControlFlow::Break(()),
        }
    }
}

impl<Fun, Val> Mappable<Fun> for bool
where
    Fun: Fn() -> Val,
{
    type Output = Option<Val>;

    fn map_response(self, f: &Fun) -> Self::Output {
        match self {
            true => Some(f()),
            false => None,
        }
    }
}

impl Combinable<()> for bool {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce(),
    {
        match self {
            true => {
                response();
                true
            }
            false => false,
        }
    }
}

impl Combinable<bool> for bool {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> bool,
    {
        self && response()
    }
}

impl<Val> Combinable<Sure<Val>> for bool {
    type Output = Option<Val>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Sure<Val>,
    {
        match self {
            true => Some(response().value()),
            false => None,
        }
    }
}

impl<Val> Combinable<Option<Val>> for bool {
    type Output = Option<Val>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Option<Val>,
    {
        match self {
            true => response(),
            false => None,
        }
    }
}

impl Disjoinable<bool> for bool {
    type Output = bool;

    fn disjoin_response<Fun, Rec, Str>(
        self,
        response: Fun,
        recover: Rec,
        stream: &mut Str,
    ) -> Self::Output
    where
        Fun: FnOnce(&mut Str) -> bool,
        Rec: FnOnce(&mut Str),
    {
        match self {
            true => true,
            false => {
                recover(stream);
                response(stream)
            }
        }
    }
}

impl Attachable for bool {
    type Output<V> = Option<V>;

    fn attach_to_response<V>(self, value: V) -> Self::Output<V> {
        self.then_some(value)
    }
}

impl Recoverable for bool {
    fn recover_response<Rec, Str>(self, on_residual: Rec, stream: &mut Str) -> Self
    where
        Rec: FnOnce(&mut Str),
    {
        match self {
            true => true,
            false => {
                on_residual(stream);
                false
            }
        }
    }
}

impl Optionable for bool {
    type Output = ();

    fn opt_response(self) -> Self::Output {
        let _ = self;
    }
}

impl Fallible for bool {
    type Infallible = ();
}
