fn main() {
    let x = 5;
    println!("Initial value of x: {}", x);

    // Shadowing x with a new value
    let x = x + 1;
    println!("After adding 1: {}", x);

    // Shadowing x again with a different type
    let x = format!("x is now a string: {}", x);
    println!("{}", x);
}
