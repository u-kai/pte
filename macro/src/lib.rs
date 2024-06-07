extern crate proc_macro;

use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    Ident, Type,
};

#[proc_macro]
pub fn atcorder_exe(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item: proc_macro2::TokenStream = item.into();
    quote! {
        fn main() {
            use core::{
                FromStrArgument,
                AcceptArgument,
                Lines,
                parse_lines,
            };
            let mut i = String::new();
            let mut result = std::io::stdin().read_line(&mut i).unwrap();
            while result > 1 {
                result = std::io::stdin().read_line(&mut i).unwrap();
            }
            let mut lines = Lines::new(&i);
            #[parse_lines(lines)]
            #item
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn parse_lines(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_lines_impl(attr.into(), item.into()).into()
}

fn parse_attr(attr: ParseStream) -> syn::Result<Ident> {
    attr.parse()
}

fn parse_lines_impl(
    lines: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let lines_ident = parse_attr.parse2(lines).unwrap();
    let fn_sig = fn_parse.parse2(item).unwrap();
    let consume_lines = fn_sig.args.iter().map(|(name, ty)| {
        if is_isize(ty) {
            quote! {
                let arg = FromStrArgument::<isize>::new();
                let #name = arg.consume(&mut #lines_ident).unwrap();
            }
        } else if is_vec(ty) {
            let ty = if let Type::Path(path) = ty {
                if let Some(segment) = path.path.segments.first() {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let syn::GenericArgument::Type(ty) = args.args.first().unwrap() {
                            ty
                        } else {
                            todo!();
                        }
                    } else {
                        todo!();
                    }
                } else {
                    todo!();
                }
            } else {
                todo!();
            };
            quote! {
                let arg = VecArgument::<#ty>::new();
                let #name = arg.consume(&mut #lines_ident).unwrap();
            }
        } else {
            todo!();
        }
    });
    let fn_sig_declare = fn_sig.to_declare_token_stream();
    let fn_sig_execute = fn_sig.to_execute_token_stream();
    quote! {
        #fn_sig_declare

        #(#consume_lines)*

        #fn_sig_execute
    }
}

struct FunctionSignature {
    name: Ident,
    args: Vec<(Ident, Type)>,
    body: syn::Block,
}

impl FunctionSignature {
    fn to_execute_token_stream(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let args = self.args.iter().map(|(name, _)| {
            quote! { #name }
        });
        quote! {
            #name(#(#args),*);
        }
    }
    fn to_declare_token_stream(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let args = self.args.iter().map(|(name, ty)| {
            quote! { #name: #ty }
        });
        let body = &self.body;
        quote! {
            fn #name(#(#args),*) #body
        }
    }
}

fn fn_parse(input: ParseStream) -> syn::Result<FunctionSignature> {
    FunctionSignature::parse(input)
}

impl Parse for FunctionSignature {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _fn: syn::Token![fn] = input.parse()?;
        let name: Ident = input.parse()?;
        let content;
        let _parentheses = syn::parenthesized!(content in input);
        let args = content
            .parse_terminated::<_, syn::Token![,]>(|input| {
                let name: Ident = input.parse()?;
                let _colon: syn::Token![:] = input.parse()?;
                let ty: Type = input.parse()?;
                Ok((name, ty))
            })?
            .into_iter()
            .collect();
        if input.peek(syn::Token![->]) {
            let _arrow: syn::Token![->] = input.parse()?;
            let _ty: Type = input.parse()?;
        }
        let body: syn::Block = input.parse().map_err(|e| {
            syn::Error::new(
                e.span(),
                format!(
                    "expected block expression for function body {}",
                    input.to_string()
                ),
            )
        })?;
        Ok(Self { name, args, body })
    }
}

fn is_vec(ty: &Type) -> bool {
    if let Type::Path(path) = ty {
        if let Some(segment) = path.path.segments.first() {
            if segment.ident == "Vec" {
                return true;
            }
        }
    }
    false
}

fn is_isize(ty: &Type) -> bool {
    if let Type::Path(path) = ty {
        if let Some(segment) = path.path.segments.first() {
            if segment.ident == "isize" {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use helper::Lines;

    use super::*;
    #[test]
    fn parse_num_and_vec_lines() {
        let s = "1 2 4
1 2 3";
        // expect expand assert_isize(1, 2, 3);
        let token = parse_lines_impl(
            quote! { lines },
            quote! {
                fn assert_isize(a: isize, b: isize, c: isize,v: Vec<isize>) {
                    assert_eq!(a, 1);
                    assert_eq!(b, 2);
                    assert_eq!(c, 3);
                    assert_eq!(v, vec![1, 2, 3]);
                }
            },
        );
        let expected = quote! {
            fn assert_isize(a: isize, b: isize, c: isize,v: Vec<isize>) {
                assert_eq!(a, 1);
                assert_eq!(b, 2);
                assert_eq!(c, 3);
                assert_eq!(v, vec![1, 2, 3]);
            }
            let arg = FromStrArgument::<isize>::new();
            let a = arg.consume(&mut lines).unwrap();
            let arg = FromStrArgument::<isize>::new();
            let b = arg.consume(&mut lines).unwrap();
            let arg = FromStrArgument::<isize>::new();
            let c = arg.consume(&mut lines).unwrap();
            let arg = VecArgument::<isize>::new();
            let v = arg.consume(&mut lines).unwrap();
            assert_isize(a, b, c,v);
        };
        assert_eq!(token.to_string(), expected.to_string());
    }
    #[test]
    fn parse_num_line() {
        // expect expand assert_isize(1, 2, 3);
        let token = parse_lines_impl(
            quote! { lines },
            quote! {
                fn assert_isize(a: isize, b: isize, c: isize) {
                    assert_eq!(a, 1);
                    assert_eq!(b, 2);
                    assert_eq!(c, 3);
                }
            },
        );
        let expected = quote! {
            fn assert_isize(a: isize, b: isize, c: isize) {
                assert_eq!(a, 1);
                assert_eq!(b, 2);
                assert_eq!(c, 3);
            }
            let arg = FromStrArgument::<isize>::new();
            let a = arg.consume(&mut lines).unwrap();
            let arg = FromStrArgument::<isize>::new();
            let b = arg.consume(&mut lines).unwrap();
            let arg = FromStrArgument::<isize>::new();
            let c = arg.consume(&mut lines).unwrap();
            assert_isize(a, b, c);
        };
        assert_eq!(token.to_string(), expected.to_string());
    }
}
