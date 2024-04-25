#![allow(unused)]
#![cfg(feature = "experimental")]

pub mod parser {
    pub(crate) mod adapters {
        pub mod and;
        pub mod as_ref;
        pub mod auto_bt;
        pub mod delimited;
        pub mod eq;
        pub mod filter;
        pub mod ignore;
        pub mod map;
        pub mod map_err;
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
pub mod response {
    pub(crate) mod adapters {
        pub mod bool;
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
pub mod stream {
    pub mod adapters;
    pub mod traits;
}
pub mod util {
    pub mod text;
}
pub mod prelude;
