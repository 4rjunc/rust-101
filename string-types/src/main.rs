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
    println!("my_str: {}", my_str);

    //Box<str>
    let my_string1 = String::from("This is rust learning repo.");
    let my_boxed_str: Box<str> = my_string1.into_boxed_str();
    println!("Box<str> : {}", my_boxed_str);
        
}
