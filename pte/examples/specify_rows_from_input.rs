use pte::pte;

// ex:
// ```shell
// cargo run
// 4
// 1 2
// 3 4 5
// 6 7
// 8 9 10
#[pte(row = in0)]
fn solve(v: Vec<Vec<isize>>) -> isize {
    v.iter().flatten().sum::<isize>()
}
