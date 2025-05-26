use std::sync::Arc;
use std::thread;

fn main() {
    // Create a reference-counted integer
    let numbers = Arc::new(vec![1, 2, 3]);

    let mut handles = vec![];

    for i in 0..3 {
        let numbers_clone = Arc::clone(&numbers); // Increment ref count

        let handle = thread::spawn(move || {
            println!("Thread {} sees: {:?}", i, numbers_clone);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads done. Original Arc still valid: {:?}", numbers);
}
