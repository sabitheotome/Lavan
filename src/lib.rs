#![allow(unused)]

pub mod parser {
    pub mod adapters {
        pub mod core {
            pub mod and;
            pub mod as_ref;
            pub mod filter;
            pub mod ignore;
            pub mod map;
            pub mod map_err;
            pub mod non_terminal;
            pub mod opt;
            pub mod or;
            pub mod repeat;
            pub mod try_map;
        }
        pub mod util {
            pub mod delimited;
            pub mod owned;
            pub mod parse_str;
            pub mod spanned;
            pub mod unwrapped;
        }
        //pub mod conversion;
    }
    pub(crate) mod prelude;
    pub mod sources;
    pub mod traits;
    pub(crate) mod util;
}
pub mod response {
    pub mod adapters {
        pub mod bool;
        pub mod option;
        pub mod result;
        pub mod sure;
        pub mod unit;
        pub mod unsure;
    }
    pub(crate) mod prelude;
    pub mod traits;
    pub mod util;
}
pub mod stream {
    pub mod adapters;
    pub mod traits;
}
pub mod prelude {
    // TODO
}
