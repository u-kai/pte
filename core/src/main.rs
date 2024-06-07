use core::atcorder_exe;

atcorder_exe!(
    fn solve(a: isize, b: isize, v: Vec<isize>, vv: Vec<Vec<isize>>) {
        let a = a + b;
        println!("{}", a);
        println!("{:?}", v);
        println!("{:?}", vv);
    }
);
