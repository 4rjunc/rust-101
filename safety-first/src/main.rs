// Import the standard pointer library
use std::ptr;

// Define a static mutable integer variable (requires unsafe blocks to modify)
static mut POINTS: i32 = 10;

// Unsafe function that demonstrates null pointer checking
unsafe fn test() {
    println!("test() function");

    // Create a null pointer of type const i32
    let ptr3: *const i32 = ptr::null();

    // Check if pointer is not null before dereferencing
    if !ptr3.is_null() {
        println!("This value is {}", *ptr3);
    }
}

// Alternative implementation with unsafe block inside a safe function
//
// ```rust
// fn test() {
//     println!("test() function");
//     let ptr3: *const i32 = ptr::null();
//     unsafe {
//         if !ptr3.is_null() {
//             println!("This value is {}", *ptr3);
//         }
//     }
// }
// ```

fn main() {
    // Create a stack variable and get a raw pointer to it
    let a = 10;
    let ptr: *const i32 = &a;

    // Dereference the pointer inside an unsafe block
    unsafe {
        println!("The values is {}", *ptr);
    }

    // Print the memory address of the pointer
    println!("The address is {:p}", ptr);

    // Create a heap-allocated value using Box
    let b = Box::new(10);

    // Get a raw pointer to the heap value
    let ptr2: *const i32 = &*b;

    // Dereference the pointer to the heap value
    unsafe {
        println!("The values is {}", *ptr2);
    }

    // Print the memory address of the heap pointer
    println!("The address is {:p}", ptr2);

    // Call the unsafe test function
    unsafe {
        test();
    }

    // Modify the static variable inside an unsafe block
    unsafe {
        POINTS = 1;
        println!("POINTS: {POINTS}")
    }
}
