use std::fmt::Display;

/// Prints a value that implements the [`Display`] trait.
///
/// # How it works
/// Rust *monomorphizes* generic functions at compile time.
/// That means the compiler generates a specialized version of this
/// function for each concrete type that is used.
///
/// For example, using this function with `&str`, `i32`, and `f64`
/// will result in code similar to:
///
/// ```ignore
/// fn print_str(x: &str) {
///     println!("Message: {}", x);
/// }
///
/// fn print_i32(x: i32) {
///     println!("Message: {}", x);
/// }
///
/// fn print_f64(x: f64) {
///     println!("Message: {}", x);
/// }
/// ```

fn print<T: Display>(x: T) {
    println!("Message: {}", x);
}

fn main() {
    print("Hello, world!");
    print(5);
    print(5.69);
}

