fn main() {
    let mut num = 5;

    // Creating raw pointers from references
    let r1 = &num as *const i32; // Immutable raw pointer
    let r2 = &mut num as *mut i32; // Mutable raw pointer

    // Raw pointers can only be dereferenced in unsafe blocks
    unsafe {
        println!("r1 is: {}", *r1); // Reading through immutable raw pointer
        println!("r2 is: {}", *r2); // Reading through mutable raw pointer

        // We can modify values through mutable raw pointers
        *r2 = 10;
        println!("After modification, r2 is: {}", *r2);
        println!("Original num is now: {}", num);
    }

    // Example showing that raw pointers don't follow borrowing rules
    let mut value = 42;

    // Unlike references, we can have both const and mut raw pointers simultaneously
    let raw_const = &value as *const i32;
    let raw_mut = &mut value as *mut i32;

    unsafe {
        println!(
            "We can read through both pointers: {} and {}",
            *raw_const, *raw_mut
        );

        // We can modify through the mutable pointer
        *raw_mut = 100;

        // And still read through the const pointer, which would be invalid with references
        println!(
            "After modification: const sees {}, mut sees {}",
            *raw_const, *raw_mut
        );
    }

    // Raw pointers can be null (references cannot)
    let null_ptr: *const i32 = std::ptr::null();

    // We can check if a pointer is null before dereferencing
    unsafe {
        if !null_ptr.is_null() {
            println!("This won't print because pointer is null");
            // Dereferencing null would cause undefined behavior
            // println!("{}", *null_ptr);  // DON'T DO THIS
        }
    }

    // Advanced example: pointer arithmetic (not possible with references)
    let array = [1, 2, 3, 4, 5];
    let ptr = &array[0] as *const i32;

    unsafe {
        // We can do pointer arithmetic to access other elements
        println!("First element: {}", *ptr);
        println!("Second element: {}", *ptr.add(1));
        println!("Third element: {}", *ptr.add(2));

        // Or we can use offsets
        println!("Fourth element: {}", *ptr.offset(3));
    }
}
