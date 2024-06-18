use pte::pte;

// ex:
// ```shell
// cargo run
// 3 4
// 1 2
// 3 4 5
// 6 7
// 8 9 10
#[pte(row = n)]
fn solve(w: usize, n: usize, v: Vec<Vec<usize>>) -> usize {
    v.iter().flatten().sum::<usize>() + w + n
}
