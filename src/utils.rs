macro_rules! print_value {
    ($x:expr) => (println!("{} = {:?}", stringify!($x), $x))
}

const EPSILON: f64 = 0.00001;

pub fn almost_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}