use std::rc::Rc;

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
        
    //Rc<str>
    let some_large_str: &'static str = "This is a Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet."; 

    // extracting subsection that multiple part of the program will reference to 
    let subsection: Rc<str> = Rc::from(&some_large_str[5..20]);
    // simulate multiple owners by cloning Rc 
    let another_reference_1 = Rc::clone(&subsection);
    let another_reference_2 = Rc::clone(&subsection);

    println!("another_reference_1: {}", another_reference_1);
    println!("another_reference_2: {}", another_reference_2);


}
