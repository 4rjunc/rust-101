use std::rc::Rc;

fn main() {
    // Rc pointing to a string
    let s = Rc::new(String::from("Hello World"));

    //Clone Rc (increase reference count)
    let s1 = Rc::clone(&s);
    let s2 = Rc::clone(&s);

    println!("Original: {}", s);
    println!("Clone 1: {}", s1);
    println!("Clone 2: {}", s2);

    // print how much reference to s
    println!("Reference count to s: {}", Rc::strong_count(&s));

    // Dropping one reference
    drop(s1);
    println!("After dropping s1, count: {}", Rc::strong_count(&s));

    drop(s2);
    println!("After dropping s2, count: {}", Rc::strong_count(&s));
}
