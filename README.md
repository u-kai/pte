# Parse input To Execute

## pte

### How to use

1. Add dependency to your `Cargo.toml`.

```toml
[dependencies]
pte = {git = "https://github.com/u-kai/atcoder-exe.git",path="./pte"}
```

1. Write your function that solves the problem. The function will take stdin as arguments and print a value to stdout.

```rust
fn solve(a: i32, b: i32) -> i32 {
    a + b
}
```

1. Use the `pte` macro and define the function you want to use.

```rust
use pte::pte;
#[pte]
fn solve(a: i32, b: i32) -> i32 {
    a + b
}
```

1. Run the code and you will be prompted to enter the input.

```shell
$ cargo run
1 2

3
```

- The input is `1 2` and next line appears to enter the input.
- The output is `3` which is the result of `1 + 2`.

## OLD

- Below code is the full code to submit to Atcoder.

```rust
macro_rules! pte {
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

pte!(solve(i, s));
```
