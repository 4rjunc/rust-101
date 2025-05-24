fn main() {
    // Initial value
    let x = 5;
    println!("Original x: {}", x);

    // Shadowing x with a new value
    let x = x + 1;
    println!("After increment: {}", x);

    // Shadowing again with a new type
    let x = format!("x is now a string: {}", x);
    println!("{}", x);
}
