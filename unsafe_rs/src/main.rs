fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        print!("r1 is: {}", *r1);
        print!("r2 is: {}", *r2);
    }
}
