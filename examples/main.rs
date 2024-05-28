macro_rules! atcoder_exe {
    ($fn:ident($($var:ident),*)) => {
       fn main() {
           let mut input = String::new();
           std::io::stdin().read_line(&mut input).unwrap();
           let mut iter = input.trim().split_whitespace();
            $(
                let $var = iter.next().unwrap().parse().unwrap();
            )*
           let result = $fn($($var),*);
           println!("{}", result);
       }
    };
}

fn solve(a: i32, b: i32) -> i32 {
    a + b
}

atcoder_exe!(solve(i, s));
