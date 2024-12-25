use crate::prelude::*;
use syn::Path;

pub struct Traits {
    pub once_parser: Path,
    pub mut_parser: Path,
    pub ref_parser: Path,
}

impl Traits {
    fn _new() -> Self {
        Self {
            once_parser: lavan_path!(parser::traits::ParseOnce),
            mut_parser: lavan_path!(parser::traits::ParseMut),
            ref_parser: lavan_path!(parser::traits::Parse),
        }
    }

    pub fn associated(ty: impl ToTokens) -> Self {
        Self {
            once_parser: lavan_path!(parser::traits::ParseOnce<#ty>),
            mut_parser: lavan_path!(parser::traits::ParseMut<#ty>),
            ref_parser: lavan_path!(parser::traits::Parse<#ty>),
        }
    }
}
