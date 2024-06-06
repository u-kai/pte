extern crate proc_macro;

use std::{
    collections::VecDeque,
    fmt::Debug,
    str::{FromStr, Split},
};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{ParseStream, Parser},
    parse_macro_input, Error, Ident, ItemFn, Type,
};

//struct Linej
//impl Line {
//    fn new(s: impl Into<String>,sep: char) -> Self {
//        Line{
//            inner: s.into(),
//            sep,
//        }
//    }
//    fn split_by(&self) -> impl Iterator<Item = &str> {
//        self.inner.split(self.sep)
//    }
//    fn to_vec<E: Debug, T: FromStr<Err = E>>(&self, sep: char) -> Vec<T> {
//        self.split_by()
//            .into_iter()
//            .map(|s| s.parse::<T>().unwrap())
//            .collect()
//    }
//    fn consume<E: Debug, T: FromStr<Err = E>>(&mut self)
//}

//trait Lines {
//    fn next(&mut self) -> Option<Line>;
//}
//
//
//
//

struct Line {
    value: Split<'static, &'static str>,
}
impl Line {
    fn new(value: &'static str) -> Self {
        Line {
            value: value.split(" "),
        }
    }
    fn next_data(&mut self) -> Option<&str> {
        self.value.next()
    }
    fn to_vec<T: FromStr>(self) -> Vec<T> {
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
mod tests {
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
    fn line_next_data() {
        let s = "1 2 345 67 8";
        let mut focus = Line::new(s);
        let data = focus.next_data();
        assert_eq!(data.unwrap(), "1");
        let data = focus.next_data();
        assert_eq!(data.unwrap(), "2");
        let data = focus.next_data();
        assert_eq!(data.unwrap(), "345");
        let data = focus.next_data();
        assert_eq!(data.unwrap(), "67");
        let data = focus.next_data();
        assert_eq!(data.unwrap(), "8");
        let data = focus.next_data();
        assert_eq!(data, None);
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
