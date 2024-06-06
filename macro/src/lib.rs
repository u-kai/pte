extern crate proc_macro;

use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    Ident, Type,
};

#[proc_macro]
pub fn atcorder_exe(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = quote! {
        fn main() {
            expand_lines_from_stdin!();
            #[parse_lines(lines)]
            fn solve(a: isize, b: isize) {
                let result = a + b;
                println!("{}", result);
            }
        }
    };
    result.into()
}

#[proc_macro_attribute]
pub fn parse_lines(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_lines_impl(attr.into(), item.into()).into()
}

macro_rules! expand_lines_from_stdin {
    () => {
        let mut input = String::new();
        let mut result = std::io::stdin().read_line(&mut input).unwrap();
        while result > 0 {
            result = std::io::stdin().read_line(&mut input).unwrap();
        }
        let mut lines = Lines::new(&input);
    };
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
    let use_lines = quote! {
        use helper::*;
    };
    let consume_lines = fn_sig.args.iter().map(|(name, ty)| {
        if is_isize(ty) {
            quote! {
                let arg = FromStrArgument::<isize>::new();
                let #name = arg.consume(&mut #lines_ident).unwrap();
            }
        } else {
            todo!();
        }
    });
    let fn_sig_declare = fn_sig.to_declare_token_stream();
    let fn_sig_execute = fn_sig.to_execute_token_stream();
    quote! {
        #use_lines
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
        let body: syn::Block = input.parse()?;
        Ok(Self { name, args, body })
    }
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
    fn parse_lines() {
        let s = "1 2 4";
        let mut lines = Lines::new(s);
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
            use helper::*;
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
    //#[test]
    //fn line_all_isize() {
    //    let s = "1 2 3";
    //    fn assert_isize(a: isize, b: isize, c: isize) {
    //        assert_eq!(a, 1);
    //        assert_eq!(b, 2);
    //        assert_eq!(c, 3);
    //    }
    //    parse_lines!(s, assert_isize);
    //}
}

/// Lines
///
use std::{
    collections::VecDeque,
    str::{FromStr, Split},
};

struct Line {
    value: Split<'static, &'static str>,
}
impl Line {
    pub fn new(value: &'static str) -> Self {
        Line {
            value: value.split(" "),
        }
    }
    pub fn next_data(&mut self) -> Option<&str> {
        self.value.next()
    }
    pub fn to_vec<T: FromStr>(self) -> Vec<T> {
        self.value
            .into_iter()
            .filter_map(|s| s.parse::<T>().ok())
            .collect()
    }
}

struct Lines {
    inner: VecDeque<Line>,
}
impl Lines {
    fn new(s: &'static str) -> Self {
        let inner = s.split("\n").map(|s| Line::new(s)).collect();
        Lines { inner }
    }
    fn next_line(&mut self) -> Option<Line> {
        self.inner.pop_front()
    }
    fn next_data(&mut self) -> Option<&str> {
        self.inner.get_mut(0).and_then(|line| line.next_data())
    }
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

trait AcceptArgument<T> {
    fn consume(&self, lines: &mut Lines) -> Option<T>;
}

struct FromStrArgument<T: FromStr> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FromStr> FromStrArgument<T> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<T: FromStr> AcceptArgument<T> for FromStrArgument<T> {
    fn consume(&self, lines: &mut Lines) -> Option<T> {
        lines.next_data().and_then(|s| s.parse().ok())
    }
}

struct VecArgument<T: FromStr> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FromStr> VecArgument<T> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

struct TwoDVecArgument<T: FromStr> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FromStr> TwoDVecArgument<T> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: FromStr> AcceptArgument<Vec<Vec<T>>> for TwoDVecArgument<T> {
    fn consume(&self, lines: &mut Lines) -> Option<Vec<Vec<T>>> {
        if lines.is_empty() {
            return None;
        }
        let mut result = Vec::new();
        while let Some(line) = lines.next_line() {
            let v = line.to_vec();
            if v.is_empty() {
                continue;
            }
            result.push(v);
        }
        Some(result)
    }
}

impl<T: FromStr> AcceptArgument<Vec<T>> for VecArgument<T> {
    fn consume(&self, lines: &mut Lines) -> Option<Vec<T>> {
        let line = lines.next_line()?;
        Some(line.to_vec())
    }
}

#[cfg(test)]
mod lines_tests {
    use super::*;
    #[test]
    fn isize_isize_two_d_vec() {
        let s = "1 2\n345 67 8\n9 10";
        let mut lines = Lines::new(s);
        let num_arg = FromStrArgument::<isize>::new();
        let two_d_vec_arg = TwoDVecArgument::<isize>::new();

        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 1);
        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 2);
        let vec = two_d_vec_arg.consume(&mut lines);
        assert_eq!(vec.unwrap(), vec![vec![345, 67, 8], vec![9, 10]]);
    }
    #[test]
    fn consume_line_number() {
        let s = "1 2 3";
        let mut lines = Lines::new(s);
        let num_arg = FromStrArgument::<isize>::new();
        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 1);
        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 2);
        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 3);
        let num = num_arg.consume(&mut lines);
        assert_eq!(num, None);
    }
    #[test]
    fn consume_line_string() {
        let s = "1 2 3";
        let mut lines = Lines::new(s);
        let str_arg = FromStrArgument::<String>::new();
        let str = str_arg.consume(&mut lines);
        assert_eq!(str.unwrap(), "1");
        let str = str_arg.consume(&mut lines);
        assert_eq!(str.unwrap(), "2");
        let str = str_arg.consume(&mut lines);
        assert_eq!(str.unwrap(), "3");
        let str = str_arg.consume(&mut lines);
        assert_eq!(str, None);
    }
    #[test]
    fn consume_vec_number() {
        let s = "1 2 3";
        let mut lines = Lines::new(s);
        let vec_arg = VecArgument::<isize>::new();
        let vec = vec_arg.consume(&mut lines);
        assert_eq!(vec.unwrap(), vec![1, 2, 3]);
        let vec = vec_arg.consume(&mut lines);
        assert_eq!(vec, None);
    }
    #[test]
    fn consume_two_d_vec_number() {
        let s = "1 2 3\n4 5 6";
        let mut lines = Lines::new(s);
        let vec_arg = TwoDVecArgument::<isize>::new();
        let vec = vec_arg.consume(&mut lines);
        assert_eq!(vec.unwrap(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let vec = vec_arg.consume(&mut lines);
        assert_eq!(vec, None);
    }
    #[test]
    fn line_next_data() {
        let s = "1 2 345 67 8";
        let mut line = Line::new(s);
        let data = line.next_data();
        assert_eq!(data.unwrap(), "1");
        let data = line.next_data();
        assert_eq!(data.unwrap(), "2");
        let data = line.next_data();
        assert_eq!(data.unwrap(), "345");
        let data = line.next_data();
        assert_eq!(data.unwrap(), "67");
        let data = line.next_data();
        assert_eq!(data.unwrap(), "8");
        let data = line.next_data();
        assert_eq!(data, None);
    }
}
