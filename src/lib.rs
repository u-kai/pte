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
}

trait AcceptArgument<T> {
    fn consume(&self, lines: &mut Lines) -> Option<T>;
}

struct NumberArgument<T: std::ops::Add> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: std::ops::Add> NumberArgument<T> {
    fn new() -> Self {
        NumberArgument {
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<T: std::ops::Add + FromStr> AcceptArgument<T> for NumberArgument<T> {
    fn consume(&self, lines: &mut Lines) -> Option<T> {
        lines.next_data().and_then(|s| s.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn line_next_data() {
        let s = "1 2 3";
        let mut focus = Line::new(s);
        let data = focus.next_data();
        assert_eq!(data.unwrap(), "1");
        let data = focus.next_data();
        assert_eq!(data.unwrap(), "2");
        let data = focus.next_data();
        assert_eq!(data.unwrap(), "3");
        let data = focus.next_data();
        assert_eq!(data, None);
    }
    #[test]
    fn consume_line_number() {
        let s = "1 2 3";
        let mut lines = Lines::new(s);
        let num_arg = NumberArgument::<isize>::new();
        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 1);
        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 2);
        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 3);
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
