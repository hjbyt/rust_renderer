macro_rules! print_value {
    ($x:expr) => (println!("{} = {:?}", stringify!($x), $x))
}
