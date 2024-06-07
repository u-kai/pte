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

1. Use the `pte` macro and pass the function name and arguments.

```rust
// Arguments names (like a and b) are free to choose, but ther must be different from each other.
use pte::pte;
#[pte(main)]
fn solve(a: i32, b: i32) -> i32 {
    a + b
}
```

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
