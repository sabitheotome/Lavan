#![allow(unused)]
#![cfg(feature = "unstable")]

#[macro_export]
macro_rules! or {
    ($first:expr $(,$tail:expr)+ $(,)?) => {
        $first$(.or($tail))+
    }
}

#[macro_export]
macro_rules! recursive {
    ($ident:ident) => {
        fn $ident()
    }
}

pub mod parser {
    pub mod adapters {
        pub mod and;
        pub mod as_ref;
        pub mod auto_bt;
        pub mod catch;
        pub mod del;
        pub mod delimited;
        pub mod eq;
        pub mod filter;
        pub mod lift;
        pub mod map;
        pub mod never_fails;
        pub mod ok;
        pub mod opt;
        pub mod or;
        pub mod owned;
        pub mod parse_str;
        pub mod persist;
        pub mod repeat;
        pub mod slice;
        pub mod spanned;
        pub mod then;
        pub mod try_with;
        pub mod unwrapped;
    }
    pub(crate) mod prelude;
    pub mod sources;
    pub mod traits;
    pub(crate) mod util;
}
pub mod response {
    pub(crate) mod adapters {
        pub mod bool;
        pub mod exception;
        pub mod option;
        pub mod result;
        pub mod sure;
        pub mod unit;
        pub mod unsure;
    }
    pub(crate) mod prelude;
    pub(crate) mod traits;
    pub(crate) mod util;
}
pub mod input {
    pub mod adapters {
        pub mod cursor;
    }
    pub mod impls;
    pub mod prelude;
    pub mod traits;
}
pub mod util {
    pub mod text;
}
pub mod prelude;
