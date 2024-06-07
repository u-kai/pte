use std::{
    collections::VecDeque,
    str::{FromStr, Split},
};

#[derive(Debug)]
pub struct Line<'a> {
    value: Split<'a, &'a str>,
}
impl<'a> Line<'a> {
    pub fn new(value: &'a str) -> Self {
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

#[derive(Debug)]
pub struct Lines<'a> {
    inner: VecDeque<Line<'a>>,
}
impl<'a> Lines<'a> {
    pub fn new(s: &'a str) -> Self {
        let inner = s.split("\n").map(|s| Line::new(s)).collect();
        Lines { inner }
    }
    pub fn next_line(&mut self) -> Option<Line<'a>> {
        self.inner.pop_front()
    }
    pub fn next_data(&mut self) -> Option<&str> {
        self.inner.get_mut(0).and_then(|line| line.next_data())
    }
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

pub trait AcceptArgument<T> {
    fn consume(&self, lines: &mut Lines) -> Option<T>;
}

pub struct FromStrArgument<T: FromStr> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FromStr> FromStrArgument<T> {
    pub fn new() -> Self {
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

pub struct VecArgument<T: FromStr> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FromStr> VecArgument<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct TwoDVecArgument<T: FromStr> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: FromStr> TwoDVecArgument<T> {
    pub fn new() -> Self {
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
        // empty line is skipped
        if lines.is_empty() {
            return None;
        }
        while let Some(line) = lines.next_line() {
            let v = line.to_vec();
            // empty line is skipped
            if !v.is_empty() {
                return Some(v);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn isize_isize_vec_two_d_vec() {
        let s = "1 2\n345 67 8\n9 10\n11 12";
        let mut lines = Lines::new(s);
        let num_arg = FromStrArgument::<isize>::new();
        let vec_arg = VecArgument::<isize>::new();
        let two_d_vec_arg = TwoDVecArgument::<isize>::new();

        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 1);
        let num = num_arg.consume(&mut lines);
        assert_eq!(num.unwrap(), 2);
        let vec = vec_arg.consume(&mut lines);
        assert_eq!(vec.unwrap(), vec![345, 67, 8]);
        let vec = two_d_vec_arg.consume(&mut lines);
        assert_eq!(vec.unwrap(), vec![vec![9, 10], vec![11, 12]]);
    }
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
