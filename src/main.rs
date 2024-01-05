//!main function
pub use rust_demo::random;

fn main() {
    println!("Hello, world!");
    println!("{}", my_sum(1, 2));


    random::float::random();
    random::int::random();
    random::string::random();
}

/// sum numbers
///  # Examples
/// ```
/// let a = 1
/// let b = 2
/// let c = my_sum(a, b)
/// assert_eq!(c, 3)
/// ```
fn my_sum(a: i32, b: i32) -> i32 {
    a + b
}