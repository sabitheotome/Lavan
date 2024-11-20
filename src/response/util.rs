pub mod macros {
    macro_rules! tryexpr {
        ($expr:expr) => {
            match $expr.branch() {
                std::ops::ControlFlow::Continue(it) => it,
                std::ops::ControlFlow::Break(res) => return tryres!(res),
            }
        };
    }
    pub(crate) use tryexpr;

    macro_rules! tryok {
        ($expr:expr) => {
            $crate::response::traits::Response::from_value($expr.into())
        };
    }
    pub(crate) use tryok;

    macro_rules! tryerr {
        ($expr:expr) => {
            $crate::response::traits::Response::from_error($expr.into())
        };
    }
    pub(crate) use tryerr;

    macro_rules! tryres {
        ($expr:expr) => {
            $crate::response::traits::Response::from_residual($expr)
        };
    }
    pub(crate) use tryres;

    macro_rules! try_break {
        ($expr:expr) => {
            match $expr.control_flow() {
                std::ops::ControlFlow::Continue(it) => it,
                std::ops::ControlFlow::Break(err) => {
                    break;
                }
            }
        };
    }
    pub(crate) use try_break;

    macro_rules! out {
        ($a:ident <$b:ty $($(, $c:ty)+)?> ) => {
           <$b as $a $(<$($c),+>)? >::Output
        };
    }
    pub(crate) use out;

    macro_rules! val {
        ($ident:ident) => {
            <$ident::Output as $crate::response::traits::Response>::Value
        };
        ($ident:ident<$param:ty>) => {
		        <$ident::Output as $crate::response::traits::Response>::WithVal<$param>
		    };
        ($ident:ident :: $assoc:ident) => {
            <$ident::$assoc as $crate::response::traits::Response>::Value
        };
        ($ident:ident<$param:ty> :: $assoc:ident) => {
			      <$ident::$assoc as $crate::response::traits::Response>::WithVal<$param>
		    };
        ($ident0:ident in $ident1:ident) => {
			      val![$ident1<val![$ident0]>]
        };
    }
    pub(crate) use val;

    macro_rules! err {
        ($ident:ident) => {
            <$ident::Output as $crate::response::traits::Response>::Error
        };
		    ($ident:ident<$param:ty>) => {
			      <$ident::Output as $crate::response::traits::Response>::WithErr<$param>
		    };
        ($ident:ident :: $assoc:ident) => {
            <$ident::$assoc as $crate::response::traits::Response>::Error
        };
		    ($ident:ident<$param:ty> :: $assoc:ident) => {
			      <$ident::$assoc as $crate::response::traits::Response>::WithErr<$param>
		    };
    }
    pub(crate) use err;

    macro_rules! lifterr {
        ($ident:ident) => {
            <$ident::Output as $crate::response::traits::Response>::Residual
        };
        ($ident:ident :: $assoc:ident) => {
            <$ident::$assoc as $crate::response::traits::Response>::Residual
        };
    }

    pub(crate) use lifterr;

    macro_rules! intoerr {
        ($($tt:tt)*) => {
            $crate::response::traits::IntoErr<$crate::response::util::err![$($tt)*]>
        };
    }

    pub(crate) use intoerr;
}

pub mod types {
    use super::super::{prelude::*, traits::Combine};

    pub type Val<R> = <R as Response>::Value;
    pub type Err<R> = <R as Response>::Error;

    pub type Comb<A, B> = <A as Combine<B>>::Output;
    pub type ErrMap<R, F> = <R as SelectErr<F>>::Output;
}
