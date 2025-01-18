fn main() {
    // wrapping
    let x = 255u8;
    let y = 1u8;
    let sum = x.wrapping_add(y);
    println!("Sum: {}", sum); // Sum: 0
    assert_eq!(sum, 0);

    // saturating
    let a: i8 = -128;
    let y: i8 = 1;
    let diff = a.saturating_sub(y);
    println!("Diff: {}", diff); // Diff: -128
    assert_eq!(diff, -128);
}
