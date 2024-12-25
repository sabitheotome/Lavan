use crate::prelude::*;
use syn::{
    AttrStyle, Generics, Ident, ItemFn, PatType, Path, ReturnType, Signature, Type, TypeParamBound,
    WherePredicate,
};

pub fn gen(attr: TokenStream, target: TokenStream) -> TokenStream {
    let ItemFn {
        attrs, sig, block, ..
    } = parse_macro_input!(target as ItemFn);

    let bound: TypeParamBound = trait_bound(lavan_path! { input::traits::Stream }).into();

    let is_mut_in_move = "mut in move" == attr.to_string();

    let Signature {
        //ident,
        mut generics,
        inputs,
        output,
        ..
    } = sig;

    if generics.where_clause.is_none() {
        generics.make_where_clause();
    }

    let mut input_ty = input_ty_setup(&mut generics, &bound)
        .expect("You can only have one generic input (full capitalized generic)");
    let mut receiver = None;

    for ele in inputs {
        match ele {
            syn::FnArg::Typed(pat) => input_ty = input_arg_setup(input_ty, pat),
            syn::FnArg::Receiver(r) => receiver = Some(r),
        }
    }

    let input_ty = input_ty.unwrap_or_else(|| default_input_ty(&mut generics, &bound));
    let receiver = receiver.expect("Provide a Self type through a receiver argument");

    let receiver_mut_token = receiver.mutability.map(|_| quote![mut]).unwrap_or_default();

    let mut self_ty = receiver
        .colon_token
        .map(|_| receiver.ty)
        .expect("Self type not provided");

    let ReturnType::Type(_, output) = output else {
        panic!("Provide a return type");
    };

    let mut func_attrs = vec![];
    let mut impl_attrs = vec![];

    for attr in attrs {
        match attr.style {
            AttrStyle::Outer => impl_attrs.push(attr),
            AttrStyle::Inner(_) => func_attrs.push(attr),
        }
    }

    let mut once_impl = quote![];
    let mut mut_impl = quote![];
    let mut const_impl = quote![];

    let mut supported_mutabilities = vec!["once"];

    if let Type::Reference(ty) = *self_ty.clone() {
        self_ty = ty.elem.clone();
        supported_mutabilities.push("mut");
        if ty.mutability.is_none() {
            supported_mutabilities.push("const");
        }
    }

    let traits = external::Traits::associated(input_ty.clone());
    let parser_trait_path_once: Path = traits.once_parser;
    let parser_trait_path_mut: Path = traits.mut_parser;
    let parser_trait_path_const: Path = traits.ref_parser;

    for mutability in supported_mutabilities {
        let parser_trait_path = match mutability {
            "once" if is_mut_in_move => parser_trait_path_mut.clone(),
            "once" => parser_trait_path_once.clone(),
            "mut" => parser_trait_path_mut.clone(),
            "const" => parser_trait_path_const.clone(),
            _ => unreachable!(),
        };

        let mut generics = generics.clone();
        fun_name1(
            &mut generics,
            mutability,
            is_mut_in_move,
            parser_trait_path,
            &parser_trait_path_once,
        );

        if let Some(where_clause) = &mut generics.where_clause {
            filter_preds(where_clause, mutability);

            let (ig, _tg, wc) = generics.split_for_impl();

            let mim_suffix = if is_mut_in_move {
                quote![.as_mut()]
            } else {
                quote![]
            };

            let common_macros = quote! {
                macro_rules! input {
                    () => (input)
                }
                macro_rules! eval {
                    ($expr:expr) => ($expr.parse_once(input))
                }
            };

            let specific_macros = fun_name2(mutability, mim_suffix);

            let body = quote! {
                #common_macros
                #specific_macros
                #block
            };

            match mutability {
                "once" => {
                    once_impl = quote! {
                        #(#impl_attrs)*
                        #[allow(non_camel_case_types)]
                        impl #ig #parser_trait_path_once for #self_ty #wc
                        {
                            type Output = #output;

                            #(#func_attrs)*
                            fn parse_once(#receiver_mut_token self, input: &mut #input_ty) -> #output {
                               #body
                            }
                        }
                    };
                }
                "mut" => {
                    mut_impl = quote! {
                        #(#impl_attrs)*
                        #[allow(non_camel_case_types)]
                        impl #ig #parser_trait_path_mut for #self_ty #wc
                        {
                            #(#func_attrs)*
                            fn parse_mut(&mut self, input: &mut #input_ty) -> #output {
                                #body
                            }
                        }
                    };
                }
                "const" => {
                    const_impl = quote! {
                        #(#impl_attrs)*
                        #[allow(non_camel_case_types)]
                        impl #ig #parser_trait_path_const for #self_ty #wc
                        {
                            #(#func_attrs)*
                            fn parse(&self, input: &mut #input_ty) -> #output {
                                #body
                            }
                        }
                    };
                }
                _ => unreachable!(),
            };
        }
    }

    quote! { #once_impl #mut_impl #const_impl }.into()
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

// TODO: PatType::attr
fn input_arg_setup(input_ty: Option<Type>, PatType { pat, ty, .. }: PatType) -> Option<Type> {
    if let syn::Pat::Ident(pat) = *pat {
        if pat.ident.to_string() != "input" {
            panic!("Function arguments are not supported with full capitalized generics.");
        }
        if input_ty.is_some() {
            panic!("Function arguments are not supported with full capitalized generics.");
        }
        return Some(ty.as_ref().clone());
    }
    input_ty
}

fn fun_name2(mutability: &str, mim_suffix: TokenStream2) -> TokenStream2 {
    match mutability {
        "once" => quote! {
            macro_rules! parser {
                (use [mut in move] => $expr:expr) => [$expr.as_mut()];
                (use [not(mut in move)] => $expr:expr) => [$expr];
                ($expr:expr) => [$expr #mim_suffix];
            }
            macro_rules! parse {
                (use [mut in move] => $expr:expr) => [$expr.as_mut().parse_once(input)];
                (use [not(mut in move)] => $expr:expr) => [$expr.parse_once(input)];
                ($e:expr) => [$e #mim_suffix .parse_once(input)];
            }
            macro_rules! when {
                (move => $expr:expr, $($tt:tt)*) => { $expr };
                (mut => $expr:expr, $($tt:tt)*) => { when!($($tt)*) };
                (const => $expr:expr, $($tt:tt)*) => { when!($($tt)*) };
                (_ => $expr:expr $(,)?) => { $expr };
            }
        },
        "mut" => quote! {
            macro_rules! parser {
                ($(use $t:tt =>)? $expr:expr) => [$expr.as_mut()];
            }
            macro_rules! parse {
                ($(use $t:tt =>)? $e:expr) => [$e.parse_mut(input)];
            }
            macro_rules! when {
                (move => $expr:expr, $($tt:tt)*) => { when!($($tt)*) };
                (mut => $expr:expr, $($tt:tt)*) => { $expr };
                (const => $expr:expr, $($tt:tt)*) => { when!($($tt)*) };
                (_ => $expr:expr $(,)?) => { $expr };
            }
        },
        "const" => quote! {
            macro_rules! parser {
                ($(use $t:tt =>)? $expr:expr) => [$expr.as_ref()];
            }
            macro_rules! parse {
                ($(use $t:tt =>)? $e:expr) => [$e.parse(input)];
            }
            macro_rules! when {
                (move => $expr:expr, $($tt:tt)*) => { when!($($tt)*) };
                (mut => $expr:expr, $($tt:tt)*) => { when!($($tt)*) };
                (const => $expr:expr, $($tt:tt)*) => { $expr };
                (_ => $expr:expr $(,)?) => { $expr };
            }
        },
        _ => unreachable!(),
    }
}

fn fun_name1(
    generics: &mut Generics,
    mutability: &str,
    is_mut_in_move: bool,
    parser_trait_path: Path,
    parser_trait_path_once: &Path,
) {
    if let Some(where_clause) = &mut generics.where_clause {
        for ele in &mut where_clause.predicates {
            if let WherePredicate::Type(pred) = ele {
                if pred
                    .bounds
                    .iter()
                    .find(|a| a.into_token_stream().to_string() == "ImplParser")
                    .is_some()
                {
                    if mutability == "once"
                        && is_mut_in_move
                        && pred
                            .bounds
                            .iter()
                            .find(|a| a.into_token_stream().to_string() == "DenyMutInMove")
                            .is_some()
                    {
                        pred.bounds.push(TypeParamBound::Trait(trait_bound(
                            parser_trait_path_once.clone().into_token_stream(),
                        )))
                    } else {
                        pred.bounds.push(TypeParamBound::Trait(trait_bound(
                            parser_trait_path.clone().into_token_stream(),
                        )))
                    }
                }
            }
        }
    }

    for ele in generics.type_params_mut() {
        let string = ele.ident.to_string();

        if string == string.to_lowercase() {
            if mutability == "once"
                && is_mut_in_move
                && ele
                    .bounds
                    .iter()
                    .find(|a| a.into_token_stream().to_string() == "DenyMutInMove")
                    .is_some()
            {
                ele.bounds.push(TypeParamBound::Trait(trait_bound(
                    parser_trait_path_once.clone().into_token_stream(),
                )))
            } else {
                ele.bounds.push(TypeParamBound::Trait(trait_bound(
                    parser_trait_path.clone().into_token_stream(),
                )))
            }

            ele.bounds = std::mem::take(&mut ele.bounds)
                .into_pairs()
                .filter(|a| a.value().into_token_stream().to_string() != "DenyMutInMove")
                .collect();
        }
    }
}

fn filter_preds(where_clause: &mut syn::WhereClause, mutability: &str) {
    punct_flat_map(&mut where_clause.predicates, |mut ele| {
        // TODO: lifetime support
        if let WherePredicate::Type(pr) = ele.value_mut() {
            is_pred_allowed(pr, mutability).then_some(ele)
        } else {
            Some(ele)
        }
    });
}

fn is_pred_allowed(pr: &mut syn::PredicateType, mutability: &str) -> bool {
    let mut found_eq = false;
    let mut found_ne = false;

    pr.lifetimes.as_mut().map(|lf| {
        punct_filter(&mut lf.lifetimes, |e| 'blk: {
            let syn::GenericParam::Lifetime(param) = e.value() else {
                break 'blk true;
            };

            let lf = param.lifetime.ident.to_string();

            match (lf.as_str(), lf == mutability) {
                ("once" | "mut" | "const", b) => {
                    if b {
                        found_eq = true;
                    } else {
                        found_ne = true;
                    }
                    false
                }
                _ => true,
            }
        });
    });

    match (found_eq, found_ne) {
        (false, true) => false,
        _ => true,
    }
}
