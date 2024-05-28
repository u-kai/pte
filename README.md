# Atcoder helper

## atcoder-exe

### How to use

1. Copy and paste below code.

```rust
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
```

1. Write your function that solves the problem. The function will take stdin as arguments and print a value to stdout.

```rust
fn solve(a: i32, b: i32) -> i32 {
    a + b
}
```

1. Use the `atcoder_exe!` macro and pass the function name and arguments.

```rust
// Arguments names (like a and b) are free to choose, but ther must be different from each other.
atcoder_exe!(solve(a, b));
```

- Below code is the full code to submit to Atcoder.

```rust
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
```
