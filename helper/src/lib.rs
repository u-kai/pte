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
        if self.is_empty() {
            return None;
        }
        self.inner.get_mut(0).and_then(|line| line.next_data())
    }
    pub fn consume<T: FromStr>(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.next_data().and_then(|s| s.parse().ok()).or_else(|| {
            self.next_line();
            self.consume()
        })
    }
    pub fn consume_to_vec<T: FromStr>(&mut self) -> Option<Vec<T>> {
        if self.is_empty() {
            return None;
        }
        while let Some(line) = self.next_line() {
            let v = line.to_vec();
            // empty line is skipped
            if !v.is_empty() {
                return Some(v);
            }
        }
        None
    }
    pub fn consume_to_two_d_vec<T: FromStr>(&mut self) -> Option<Vec<Vec<T>>> {
        if self.is_empty() {
            return None;
        }
        let mut result = Vec::new();
        while let Some(line) = self.next_line() {
            let v = line.to_vec();
            if v.is_empty() {
                continue;
            }
            result.push(v);
        }
        Some(result)
    }
    pub fn extend(&mut self, s: &'a str) {
        let inner = s.split("\n").map(|s| Line::new(s)).collect::<Vec<_>>();
        self.inner.extend(inner);
    }
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn isize_isize_vec_two_d_vec() {
        let s = "1 2\n345 67 8\n9 10\n11 12";
        let mut lines = Lines::new(s);
        let num = lines.consume::<isize>();
        assert_eq!(num.unwrap(), 1);
        let num = lines.consume::<isize>();
        assert_eq!(num.unwrap(), 2);
        let vec = lines.consume_to_vec::<isize>();
        assert_eq!(vec.unwrap(), vec![345, 67, 8]);
        let vec = lines.consume_to_two_d_vec::<isize>();
        assert_eq!(vec.unwrap(), vec![vec![9, 10], vec![11, 12]]);
    }
    #[test]
    fn consume_multi_line() {
        let s = "1\n2 3\ntest";
        let mut lines = Lines::new(s);
        let num = lines.consume::<isize>();
        assert_eq!(num.unwrap(), 1);
        let num = lines.consume::<isize>();
        assert_eq!(num.unwrap(), 2);
        let num = lines.consume::<isize>();
        assert_eq!(num.unwrap(), 3);
        let str = lines.consume::<String>();
        assert_eq!(str.unwrap(), "test");
        let num = lines.consume::<isize>();
        assert_eq!(num, None);
    }
    #[test]
    fn consume_line_number() {
        let s = "1 2 3";
        let mut lines = Lines::new(s);
        let num = lines.consume::<isize>();
        assert_eq!(num.unwrap(), 1);
        let num = lines.consume::<isize>();
        assert_eq!(num.unwrap(), 2);
        let num = lines.consume::<isize>();
        assert_eq!(num.unwrap(), 3);
        let num = lines.consume::<isize>();
        assert_eq!(num, None);
    }
    #[test]
    fn consume_line_string() {
        let s = "1 2 3";
        let mut lines = Lines::new(s);
        let str = lines.consume::<String>();
        assert_eq!(str.unwrap(), "1");
        let str = lines.consume::<String>();
        assert_eq!(str.unwrap(), "2");
        let str = lines.consume::<String>();
        assert_eq!(str.unwrap(), "3");
        let str = lines.consume::<String>();
        assert_eq!(str, None);
    }
    #[test]
    fn consume_vec_number() {
        let s = "1 2 3";
        let mut lines = Lines::new(s);
        let vec = lines.consume_to_vec::<isize>();
        assert_eq!(vec.unwrap(), vec![1, 2, 3]);
        let vec = lines.consume_to_vec::<isize>();
        assert_eq!(vec, None);
    }
    #[test]
    fn consume_two_d_vec_number() {
        let s = "1 2 3\n4 5 6";
        let mut lines = Lines::new(s);
        let vec = lines.consume_to_two_d_vec::<isize>();
        assert_eq!(vec.unwrap(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let vec = lines.consume_to_two_d_vec::<isize>();
        assert_eq!(vec, None);
    }
    #[test]
    fn lines_can_extends() {
        let s = "1 2 3\n4 5 6";
        let mut lines = Lines::new(s);
        let data = lines.consume::<isize>();
        assert_eq!(data.unwrap(), 1);
        let extend = "7 8 9\n10 11 12";
        lines.extend(extend);
        let data = lines.consume::<isize>();
        assert_eq!(data.unwrap(), 2);
        let data = lines.consume::<isize>();
        assert_eq!(data.unwrap(), 3);
        let v = lines.consume_to_vec::<isize>();
        assert_eq!(v.unwrap(), vec![4, 5, 6]);
        let v = lines.consume_to_two_d_vec::<isize>();
        assert_eq!(v.unwrap(), vec![vec![7, 8, 9], vec![10, 11, 12]]);
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
