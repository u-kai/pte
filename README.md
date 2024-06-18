# Parse input To Execute

## pte

pte is a macro to parse input to execute the function.

### How to use

1. Add dependency to your `Cargo.toml`.

   ```toml
   [dependencies]
   pte = {git = "https://github.com/u-kai/pte.git"}
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

- The input is `1 2` and the output is `3` which is the result of `1 + 2`.

### Specifying the Number of Input Rows

- If you want to specify the number of input rows, you can write the code like this:

```rust
use pte::pte;
#[pte(row = 2)]
fn solve(a: i32, b: i32,v:Vec<char>) -> i32 {
    a + b + v.len() as i32
}
```

```shell
cargo run
1 2   # a and b
6 7 8 # v
```

- If you want to specify the number of input rows from first input line, you can use the `pte!` macro.
- For example, input is below, first line is `3 2`, 3 is the number of columns and 2 is the number of rows.

```shell
cargo run
3 2   # 3 is skip and 2 is mapped number of rows
a b c # read and mapped to v
d e f # read and mapped to v
```

- You can write the code like below.

```rust
use pte::pte;
#[pte(row = in1)]
fn solve(v:Vec<Vec<char>>) -> i32 {
   v.len() as i32
}
```

- `in` means input and `1` means the index of the first input line.
- You couldn't specify the number of columns, because the number of columns is automatically detected.
- If you didn't specify the number of rows, default is 1.

- If you want to specify the number of input rows from input variable, you can write the code like this:

```rust
use pte::pte;
#[pte(row = n)]
fn solve(w:usize,n:usize,v:Vec<Vec<char>>) -> i32 {
   v.len() as i32
}
```

- n is function argument and the number of rows is specified by n.
- For example, input is below, first line is `3 2`, 3 is `w`, 2 is `n`. So, w,n mapped to `w` and `n` and `v` is mapped to 2 rows.

```shell
cargo run
3 2
a b c # read and mapped to v
d e f # read and mapped to v
```

## OLD Ver

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
