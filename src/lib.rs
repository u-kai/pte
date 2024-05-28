// This is a library crate
#[macro_export]
macro_rules! atcoder_exe {
    ($fn:ident($($var:ident),*)) => {
       fn main() {
           parse_stdin!($($var),*);
           let result = $fn($($var),*);
           println!("{}", result);
       }
    };
}
#[macro_export]
macro_rules! parse_stdin {
    // type suggestion
    ($($input:ident),*) => {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        vars_expanded!(iter, $($input),*);
    };
    // variables have different types
    ({$($input:ident: $t:ty),*}) => {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        vars_expanded!(iter, {$($input: $t),*});
    };
    // all variables are same type
    ($($input:ident),*,<$t:ty>) => {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        vars_expanded!(iter, {$($input),*}, <$t>);
    };
}

#[macro_export]
macro_rules! vars_expanded {
    // type suggestion
    ($iter:ident,$($input:ident),*) => {
        $(
            let $input = $iter.next().unwrap().parse().unwrap();
        )*
    };
    // variables have different types
    ($iter:ident,{$($input:ident: $t:ty),*}) => {
        $(
            let $input = $iter.next().unwrap().parse::<$t>().unwrap();
        )*
    };
    // all variables are same type
    ($iter:ident,{$($input:ident),*},<$t:ty>) => {
        $(
            let $input = $iter.next().unwrap().parse::<$t>().unwrap();
        )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_suggest() {
        let mut iter = vec!["1", "2", "3"].into_iter();
        fn assert_isize(a: isize, b: isize) {
            assert_eq!(a, b);
        }
        fn assert_string(a: String, b: String) {
            assert_eq!(a, b);
        }
        vars_expanded!(iter, a, b, c);
        assert_isize(a, 1);
        assert_isize(b, 2);
        assert_isize(c, 3);

        let mut iter = vec!["1", "2", "3"].into_iter();
        vars_expanded!(iter, a, b, c);
        assert_isize(a, 1);
        assert_isize(b, 2);
        assert_string(c, "3".to_string());
    }
    #[test]
    fn all_same_type() {
        let mut iter = vec!["1", "2", "3"].into_iter();
        vars_expanded!(iter, {a, b, c}, <String>);
        assert_eq!(a, "1");
        assert_eq!(b, "2");
        assert_eq!(c, "3");
    }
    #[test]
    fn different_types() {
        let mut iter = vec!["1", "2", "Hello"].into_iter();
        vars_expanded!(iter, {a:isize, b:isize, c:String});
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, "Hello");
    }
}
