use pte::pte;
// ex:
// ```shell
// cargo run
// 1 2
// 3 4 5
// 6 7
// 8 9
#[pte(row = 4)]
fn solve(a: isize, b: isize, v: Vec<isize>, vv: Vec<Vec<isize>>) -> isize {
    a + b + v.iter().sum::<isize>() + vv.iter().flatten().sum::<isize>()
}

#[test]
fn test_solve() {
    let a = 1;
    let b = 2;
    let v = vec![3, 4, 5];
    let vv = vec![vec![6, 7], vec![8, 9]];
    assert_eq!(solve(a, b, v, vv), 45);
}
