pub use helper::Lines;
pub use r#macro::pte;

// TODO:declare macro version
#[macro_export]
macro_rules! d_pte {
    (fn $fn_name:ident($($args:ident:$ty:ty),*) -> $ret_ty:ty  $body:block) => {
        fn $fn_name($($args:$ty),*) -> $ret_ty $body

        use pte::Lines;

        fn main() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut lines = Lines::new(&input);
            $(
                let $args = lines.consume::<$ty>().unwrap();
            )*
            let result = $fn_name($($args),*);
            println!("{}", result);
        }
    };
    ($row:literal,fn $fn_name:ident($($args:ident:$ty:ty),*) -> $ret_ty:ty  $body:block) => {
        fn $fn_name($($args:$ty),*) -> $ret_ty $body

        use pte::Lines;

        fn main() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut lines = Lines::new(&input);
            $(
                let $args = lines.consume::<$ty>().unwrap();
            )*
            let result = $fn_name($($args),*);
            println!("{}", result);
        }
    };
}
