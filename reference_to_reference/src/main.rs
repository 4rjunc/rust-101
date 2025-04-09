fn main() {
    // Create an integer variable with value 5
    let num = 5;
    // Print the memory address of 'num' using the {:p} format specifier
    println!("num: {:p}", &num);

    // Create a reference (r1) to 'num'
    let r1 = &num;
    // Create a reference (r2) to 'r1' - this is a reference to a reference
    let r2 = &r1;

    // Print the values stored in r1 and r2
    // r1 contains the address of 'num'
    // r2 contains the address of 'r1'
    println!("r1:{:p}, r2:{:p}", r1, r2);

    // Print the addresses of the references themselves
    // &r1 is the address where r1 is stored
    // &r2 is the address where r2 is stored
    println!("Addresses of references: \n r1:{:p}, r2:{:p}", &r1, &r2);
}
