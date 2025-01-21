fn main() {
    let a: u32 = 10;

    // casting a into u64
    let b = a as u64;

    // You can use `_` as the target type
    // if it can be correctly inferred
    // by the compiler. For example:
    let c: u64 = a as _;

    // Use it exclusively for going from a smaller type to a larger type.
    //
    //
    let x: u16 = 255 + 1;
    let y = x as i8;
    println!("x: {}, y: {}", x, y);
}
