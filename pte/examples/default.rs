use pte::pte;
// ex:
// ```shell
// cargo run
// 1 2 3
#[pte]
fn solve(a: isize, b: isize, c: isize) -> isize {
    a + b + c
}
