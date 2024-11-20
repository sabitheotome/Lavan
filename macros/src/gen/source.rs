use crate::prelude::*;
use syn::{Generics, Ident, ItemFn, Type, TypeParamBound};

pub fn gen(_attr: TokenStream, target: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
        ..
    } = parse_macro_input!(target as ItemFn);

    let mut inner_sig = sig.clone();

    if sig.generics.where_clause.is_none() {
        sig.generics.make_where_clause();
    }
    let lavan_crate = lavan_crate();
    let bound: TypeParamBound = trait_bound(quote! { #lavan_crate::input::traits::Stream }).into();

    let mut receiver = None;

    // remove receiver for outer function while gathering Self
    punct_filter(&mut sig.inputs, |ele| match ele.value() {
        syn::FnArg::Receiver(r) => {
            receiver = Some(r.clone());
            false
        }
        _ => true,
    });

    // remove arguments for inner function
    punct_filter(&mut inner_sig.inputs, |ele| match ele.value() {
        syn::FnArg::Receiver(_) => true,
        _ => false,
    });

    let receiver = receiver.expect("Provide a Self type through a receiver argument");

    let mut self_ty = receiver
        .colon_token
        .map(|_| receiver.ty)
        .expect("Self type not provided");

    if let Type::Reference(ty) = *self_ty.clone() {
        self_ty = ty.elem.clone();
    }

    let input_ty = input_ty_setup(&mut sig.generics, &bound)
        .expect("You can only have one generic input (full capitalized generic)");
    let _ = input_ty.unwrap_or_else(|| default_input_ty(&mut sig.generics, &bound));

    sig.output = parse_quote!(-> #lavan_crate::parser::sources::adapters::Src<#self_ty, INPUT>);

    quote! {

        #vis #sig
        {
            macro_rules! implement {
                ($expr:expr) => {
                    #(#attrs)*
                    #[parser_fn]
                    #inner_sig
                    {
                        $expr
                    }
                }
            }

           #lavan_crate::parser::sources::functions::src(#block)
        }

    }
    .into()
}

fn input_ty_setup(generics: &mut Generics, bound: &TypeParamBound) -> Result<Option<Type>, ()> {
    Ok(unique_upper_param(generics)?.map(|param| {
        if param.bounds.is_empty() {
            param.bounds.push(bound.clone());
        }

        let ident = param.ident.clone();
        parse_quote!(#ident)
    }))
}

fn default_input_ty(generics: &mut Generics, bound: &TypeParamBound) -> Type {
    let ident: Ident = parse_quote!(INPUT);
    generics
        .params
        .push(type_param(ident.clone(), [bound.clone()]));
    parse_quote!(#ident)
}
