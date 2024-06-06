use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput};

#[proc_macro_derive(ParserAdapter)]
pub fn parser_adapter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let (ig, tg, wc) = input.generics.split_for_impl();
    let predicates = wc
        .map(|e| e.predicates.clone())
        .unwrap_or_else(|| Punctuated::new());

    quote! {
        impl #ig Parser<<#ident #tg as Operator>::Scanner>
            for #ident #tg
        where
            Self: Operator,
            #predicates
        {
            type Output = <#ident #tg as Operator>::Response;
            type Operator = Self;

            fn operator(self) -> Self::Operator {
                self
            }
        }
    }
    .into()
}
