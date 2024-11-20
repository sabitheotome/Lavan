use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ImplItem, ItemImpl};

mod gen {
    pub(crate) mod func;
    pub(crate) mod impl_block;
    pub(crate) mod source;
}
mod caching;
mod external;
mod prelude;
mod util;

#[proc_macro_attribute]
pub fn parser_fn(attr: TokenStream, target: TokenStream) -> TokenStream {
    caching::cached(attr, target, gen::func::gen).unwrap()
}

#[proc_macro_attribute]
pub fn source_parser(attr: TokenStream, target: TokenStream) -> TokenStream {
    caching::cached(attr, target, gen::source::gen).unwrap()
}

#[proc_macro_attribute]
pub fn parser_impl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);
    let impl_attrs = input.attrs;
    let ty = input.self_ty;
    let (ig, _tg, wc) = input.generics.split_for_impl();
    let predicates = wc
        .map(|e| e.predicates.clone())
        .unwrap_or_else(|| Punctuated::new());

    let mut fns = vec![];
    let mut tys = vec![];

    for e in input.items {
        match e {
            ImplItem::Fn(f) => {
                fns.push(f);
            }
            ImplItem::Type(ty) => {
                tys.push(ty);
            }
            _ => {
                panic!("Not supported (1)");
            }
        }
    }

    if fns.len() != 1 {
        panic!("Not supported (2)");
    }

    if tys.len() != 2 {
        panic!("Not supported (3)");
    }

    let mut response = tys.pop().unwrap();
    let mut scanner = tys.pop().unwrap();

    if scanner.ident.to_string() != "Input" || response.ident.to_string() != "Output" {
        if response.ident.to_string() != "Input" || scanner.ident.to_string() != "Output" {
            panic!("Not supported (4)");
        } else {
            std::mem::swap(&mut scanner, &mut response);
        }
    }

    let scanner = scanner.ty;
    let response = response.ty;
    let func = fns.first().unwrap();

    let func_attrs = func.attrs.clone();
    let func_code = func.block.clone();

    quote! {
        #(#impl_attrs)*
        impl #ig Parser<#scanner> for #ty
        where
            #predicates
        {
            type Output = #response;

            #(#func_attrs)*
            fn parse_next(&self, input: &mut #scanner) -> Self::Output {
                #func_code
            }
        }
    }
    .into()
}
