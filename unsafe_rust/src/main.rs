fn main() {
    let ptr = &mut 10u8 as *mut u8;
    unsafe {
        let val_back = *ptr;
        if val_back > 0 {
            *ptr = 0;
        } else {
            *ptr = 1;
        }
    }
    println!("Hello, world!");
}
