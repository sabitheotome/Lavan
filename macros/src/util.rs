use crate::prelude::*;
use std::{result::Result, sync::OnceLock};
use syn::{punctuated::*, *};

const PRIMARY_CRATE_NAME: &str = "lavan";
static IS_PRIMARY_CRATE: OnceLock<bool> = OnceLock::new();

pub(crate) fn lavan_crate() -> TokenStream2 {
    if *IS_PRIMARY_CRATE
        .get_or_init(|| std::env::var("CARGO_PKG_NAME").unwrap() == PRIMARY_CRATE_NAME)
    {
        quote! { crate }
    } else {
        quote! { lavan }
    }
}

pub(crate) fn __private_lavan_path(ts: TokenStream2) -> TokenStream2 {
    let lavan_crate = lavan_crate();
    quote! { #lavan_crate :: #ts }
}

macro_rules! lavan_path {
    ($($tt:tt)+) => {{
        let tt = crate::util::__private_lavan_path(quote::quote!($($tt)+));
        syn::parse_quote!(#tt)
    }};
}
pub(crate) use lavan_path;

pub(crate) fn punct_filter<T, P, F>(punct: &mut Punctuated<T, P>, f: F)
where
    F: FnMut(&Pair<T, P>) -> bool,
{
    *punct = std::mem::take(punct).into_pairs().filter(f).collect();
}

pub(crate) fn punct_flat_map<T, P, U, F>(punct: &mut Punctuated<T, P>, f: F)
where
    U: IntoIterator<Item = Pair<T, P>>,
    F: FnMut(Pair<T, P>) -> U,
{
    *punct = std::mem::take(punct).into_pairs().flat_map(f).collect();
}

pub(crate) fn trait_bound(path: TokenStream2) -> TraitBound {
    TraitBound {
        paren_token: None,
        modifier: syn::TraitBoundModifier::None,
        lifetimes: None,
        path: parse_quote! { #path },
    }
}

pub(crate) fn type_param(
    ident: Ident,
    bounds: impl IntoIterator<Item = TypeParamBound>,
) -> syn::GenericParam {
    syn::GenericParam::Type(TypeParam {
        attrs: vec![],
        ident,
        bounds: Punctuated::from_iter(bounds),
        eq_token: None,
        colon_token: None,
        default: None,
    })
}

pub(crate) fn unique_upper_param(generics: &mut Generics) -> Result<Option<&mut TypeParam>, ()> {
    let mut found = None;

    for (idx, ele) in generics.type_params_mut().enumerate() {
        let string = ele.ident.to_string();
        let len = string.len();

        if len > 2 && string.chars().all(|c| c.is_uppercase()) {
            if found.is_some() {
                return Err(());
            }
            found = Some(idx);
        }
    }

    Ok(found.map(|idx| generics.type_params_mut().nth(idx).unwrap()))
}
