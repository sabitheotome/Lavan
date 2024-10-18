#![allow(unused)]
#![cfg(feature = "experimental")]

#[macro_export]
macro_rules! or {
    ($first:expr $(,$tail:expr)+ $(,)?) => {
        $first$(.or($tail))+
    }
}

fn a() {
    use crate::prelude::*;

    parser::sources::nop()
        .repeat_exact(0)
        .parse_once(&mut "fuck".chars());
}

pub mod parser {
    pub mod adapters {
        pub mod and;
        pub mod as_ref;
        pub mod auto_bt;
        pub mod delimited;
        pub mod discard;
        pub mod eq;
        pub mod filter;
        pub mod infer;
        pub mod map;
        pub mod map_err;
        pub mod never_fails;
        pub mod ok;
        pub mod opt;
        pub mod or;
        pub mod owned;
        pub mod parse_str;
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
pub mod output {
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
