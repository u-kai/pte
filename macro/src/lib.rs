extern crate proc_macro;

use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    spanned::Spanned,
    Ident, Type,
};

fn dependencies() -> proc_macro2::TokenStream {
    quote! {
        use pte::{
            Lines,
            parse_lines_and_println,
        };
    }
}

#[proc_macro_attribute]
pub fn pte(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    pte_impl(attr.into(), item.into()).into()
}

fn pte_impl(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let attr_str = attr.to_string();
    if attr_str == "" {
        return pte_main(item.into(), 1);
    }
    let dependencies = dependencies();
    if attr_str.contains("rows") {
        let n = attr_str
            .split("=")
            .last()
            .unwrap()
            .trim_start()
            .parse::<isize>()
            .unwrap();
        pte_main(item.into(), n)
    } else {
        let lit_str = parse_lit.parse2(attr).unwrap();
        quote! {
            #dependencies
            let mut lines = Lines::new(#lit_str);
            #[parse_lines(lines)]
            #item
        }
    }
}

fn read_stdin(n: isize) -> proc_macro2::TokenStream {
    let n_lit = n.to_string();
    let n_lit: proc_macro2::Literal = syn::parse_str(&n_lit).unwrap();
    quote! {
        let mut i = String::new();
         //for _ in 0..#n_lit {
         while let Ok(size) = std::io::stdin().read_line(&mut i){
            println!("{}", size);
            if size == 0 {
                break;
            }
         }
        //}
        let mut lines = Lines::new(&i);
    }
}

fn pte_main(item: proc_macro2::TokenStream, n: isize) -> proc_macro2::TokenStream {
    let dependencies = dependencies();
    let read_stdin = read_stdin(n);
    quote! {
        #dependencies
        fn main() {
            #read_stdin
            #[parse_lines_and_println(lines)]
            #item
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn parse_lines_and_println(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_lines_and_println_impl(attr.into(), item.into()).into()
}

fn parse_attr(attr: ParseStream) -> syn::Result<Ident> {
    attr.parse()
}
fn parse_lit(attr: ParseStream) -> syn::Result<syn::Lit> {
    attr.parse()
}

fn parse_lines_and_println_impl(
    lines: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let lines_ident = parse_attr.parse2(lines).unwrap();
    let fn_sig = fn_parse.parse2(item).unwrap();
    let consume_lines = fn_sig.to_consume_lines_token_stream(lines_ident);
    let fn_sig_declare = fn_sig.to_declare_token_stream();
    let fn_sig_execute = fn_sig.to_execute_token_stream();
    quote! {
        #fn_sig_declare

        #consume_lines

        let result = #fn_sig_execute
        println!("{}", result);
    }
}

fn get_vec_type(ty: &Type) -> syn::Result<&Type> {
    let Type::Path(path) = ty else {
        return Err(syn::Error::new(ty.span(), "expected path"));
    };
    let Some(segment) = path.path.segments.first() else {
        return Err(syn::Error::new(ty.span(), "expected segment"));
    };
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return Err(syn::Error::new(ty.span(), "expected angle bracketed"));
    };
    let syn::GenericArgument::Type(ty) = args.args.first().unwrap() else {
        return Err(syn::Error::new(ty.span(), "expected type"));
    };
    Ok(ty)
}

struct FunctionSignature {
    name: Ident,
    args: Vec<(Ident, Type)>,
    return_type: proc_macro2::TokenStream,
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
        let ty = &self.return_type;
        let body = &self.body;
        quote! {
            fn #name(#(#args),*) #ty #body
        }
    }
    fn to_consume_lines_token_stream(&self, lines_ident: Ident) -> proc_macro2::TokenStream {
        let result = self.args.iter().map(|(name, ty)| {
            if is_vec(ty) {
                let ty = get_vec_type(ty).map_err(|e| e.to_compile_error()).unwrap();
                if is_vec(ty) {
                    let ty = get_vec_type(ty).map_err(|e| e.to_compile_error()).unwrap();
                    return quote! {
                        let #name = #lines_ident.consume_to_two_d_vec::<#ty>().unwrap();
                    };
                }
                return quote! {
                    let #name = #lines_ident.consume_to_vec::<#ty>().unwrap();
                };
            }
            quote! {
                let #name = #lines_ident.consume::<#ty>().unwrap();
            }
        });
        quote! {
            #(#result)*
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

        let return_type = if input.peek(syn::Token![->]) {
            let _arrow: syn::Token![->] = input.parse()?;
            let return_type: Type = input.parse()?;
            quote! { -> #return_type }
        } else {
            quote! {}
        };
        let body: syn::Block = input.parse().map_err(|e| {
            syn::Error::new(
                e.span(),
                format!(
                    "expected block expression for function body {}",
                    input.to_string()
                ),
            )
        })?;

        Ok(Self {
            name,
            args,
            return_type,
            body,
        })
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

#[derive(Debug)]
struct PteAttrParser<'a> {
    attr: &'a str,
}

impl PteAttrParser<'_> {
    const ROW_KEY: &'static str = "row";
    fn new(attr: &str) -> PteAttrParser {
        PteAttrParser { attr }
    }
    fn exist_row_num_at_input(&self) -> bool {
        self.attr.contains(Self::ROW_KEY)
    }
    // get by default or row = NUMBER
    fn get_row_num(&self) -> Result<isize, String> {
        if self.attr == "" || !self.attr.contains(Self::ROW_KEY) {
            return Ok(self.default_row_num());
        }
        let row_value = self.get_row_attr_value();
        row_value.parse::<isize>().map_err(|e| e.to_string())
    }
    fn get_row_attr_value(&self) -> &str {
        if self.attr == "" || !self.attr.contains(Self::ROW_KEY) {
            return "";
        }
        let mut attrs = self.attr.split(",");
        let row_attr = attrs
            .find(|attr| attr.contains(Self::ROW_KEY))
            .unwrap_or_default();
        let split = row_attr.split("=");
        split.last().unwrap_or_default().trim()
    }
    fn get_row_num_from_input(&self, input_rows: &str) -> Result<isize, String> {
        let row_value = self.get_row_attr_value();
        let input_ref = parse_input_ref(row_value)?;
        let Some(first_line) = input_rows.lines().next() else {
            return Err("input rows is empty".to_string());
        };
        let values = first_line.split_whitespace().collect::<Vec<&str>>();
        let Ok(n) = values[input_ref].parse::<isize>() else {
            return Err(format!("invalid input reference {}", input_ref));
        };
        Ok(n)
    }
    fn default_row_num(&self) -> isize {
        1
    }
}

fn parse_input_ref(input_ref: &str) -> Result<usize, String> {
    fn error_msg(v: &str) -> String {
        format!("invalid input reference {}, format is \"inNUMBER\"", v)
    }
    let Some("in") = input_ref.get(0..2) else {
        return Err(error_msg(input_ref));
    };
    let Ok(result) = input_ref[2..3].parse::<usize>() else {
        return Err(error_msg(input_ref));
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_attr_row_from_input() {
        let attr = "row = in0, column = in1";
        let sut = PteAttrParser::new(attr);
        let input_row = "3 2\n";
        assert!(sut.exist_row_num_at_input());
        let got = sut.get_row_num_from_input(input_row).unwrap();
        assert_eq!(got, 3);
    }
    #[test]
    fn parse_attr_row_default() {
        let attr = "";
        let sut = PteAttrParser::new(attr);
        let input_row = "1 2\n";

        assert!(!sut.exist_row_num_at_input());
        let got = sut.get_row_num().unwrap();
        assert_eq!(got, 1);
    }
    #[test]
    fn parse_two_d_vec_lines() {
        // expect expand assert_isize(1, 2, 3);
        let token = parse_lines_and_println_impl(
            quote! { lines },
            quote! {
                fn assert_isize(two_d: Vec<Vec<isize>>) {
                }
            },
        );
        let expected = quote! {
            fn assert_isize(two_d: Vec<Vec<isize> >) {
            }
            let two_d = lines.consume_to_two_d_vec::<isize>().unwrap();
            let result = assert_isize(two_d);
            println!("{}", result);
        };
        assert_eq!(token.to_string(), expected.to_string());
    }
    #[test]
    fn parse_num_and_vec_lines() {
        // expect expand assert_isize(1, 2, 3);
        let token = parse_lines_and_println_impl(
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
            let a = lines.consume::<isize>().unwrap();
            let b = lines.consume::<isize>().unwrap();
            let c = lines.consume::<isize>().unwrap();
            let v = lines.consume_to_vec::<isize>().unwrap();
            let result = assert_isize(a, b, c,v);
            println!("{}", result);
        };
        assert_eq!(token.to_string(), expected.to_string());
    }
    #[test]
    fn parse_num_line() {
        // expect expand assert_isize(1, 2, 3);
        let token = parse_lines_and_println_impl(
            quote! { lines },
            quote! {
                fn add(a: isize, b: isize, c: isize) -> isize {
                    a + b + c
                }
            },
        );
        let expected = quote! {
            fn add(a: isize, b: isize, c: isize) -> isize {
                a + b + c
            }
            let a = lines.consume::<isize>().unwrap();
            let b = lines.consume::<isize>().unwrap();
            let c = lines.consume::<isize>().unwrap();
            let result = add(a, b, c);
            println!("{}", result);
        };
        assert_eq!(token.to_string(), expected.to_string());
    }
}
