fn main() {
    // String 
    // Heap Allocated
    // Ownable 
    // Growable
    // UTF-8 encoded    
    let my_string = String::from("Hello World!");
    println!("my_string: {}", my_string);

    // &str 
    let my_str: &str = &my_string;
    println!("my_str: {}", my_str)
}
