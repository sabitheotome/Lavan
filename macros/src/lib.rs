use proc_macro::TokenStream;
use quote::quote;
use std::{
    hash::{Hash, Hasher},
    io::{BufRead, Write},
    os::unix::fs::MetadataExt,
    str::FromStr,
};
use syn::{parse_macro_input, punctuated::Punctuated, ImplItem, ItemImpl};

mod func;

pub(crate) fn cached(
    attr: TokenStream,
    ts: TokenStream,
    f: impl FnOnce(TokenStream, TokenStream) -> TokenStream,
) -> std::io::Result<TokenStream> {
    const CACHE_PATH: &str = "target/cache/lavan_proc_macro";
    let ref latest = format!("{CACHE_PATH}/latest");

    let string = attr.to_string() + &ts.to_string();
    let mut hasher = std::hash::DefaultHasher::new();
    string.hash(&mut hasher);
    let hash = hasher.finish();
    let file_content = std::fs::read(format!("{CACHE_PATH}/{hash}"));

    Ok(match file_content {
        Ok(content) => {
            TokenStream::from_str(&String::from_utf8(content).expect("erro0")).expect("error1")
        }
        Err(_) => {
            let output = f(attr, ts);
            std::fs::create_dir_all(CACHE_PATH)?;

            let now = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            std::fs::File::create(latest)?;
            let ctime = std::fs::metadata(latest)?.mtime();

            if now - ctime as u64 >= 60 {
                if let Ok(mut file) = std::fs::File::open(latest) {
                    let allowed = std::io::BufReader::new(&mut file)
                        .lines()
                        .map(|e| e.expect("error2"))
                        .collect::<Vec<_>>();
                    for file in std::fs::read_dir(CACHE_PATH)? {
                        let file = file
                            .expect("error3")
                            .file_name()
                            .to_string_lossy()
                            .into_owned();
                        if !allowed.contains(&file) {
                            std::fs::remove_file(format!("{CACHE_PATH}/{file}"))?;
                            std::fs::File::create(latest)?;
                        }
                    }
                    drop(file);
                }
            }

            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(latest)?;
            file.write(hash.to_string().as_bytes())?;
            file.write(b"\n")?;
            file.flush()?;

            std::fs::write(format!("{CACHE_PATH}/{hash}"), output.to_string())?;
            output
        }
    })
}

#[proc_macro_attribute]
pub fn parser_fn(attr: TokenStream, target: TokenStream) -> TokenStream {
    cached(attr, target, func::gen).unwrap()
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
