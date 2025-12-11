use std::fmt::{Debug, Display};

fn show<T: Display + Debug>(item: T) {
    println!("Show: {}", item);
    println!("Show: {:?}", item);
}

fn main() {
    let x = 5;
    show(x);
    println!("Hello, world!");
}
