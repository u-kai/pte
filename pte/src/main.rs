use pte::pte;

#[pte(row = in1)]
fn solve(v: Vec<Vec<char>>) -> i32 {
    println!("{:?}", v);
    v.iter().map(|x| x.len()).sum::<usize>() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let v = vec![vec!['a', 'b'], vec!['c', 'd']];
        assert_eq!(super::solve(v), 4);
    }
}
