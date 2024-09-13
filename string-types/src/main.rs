use std::borrow::Cow;
use std::sync::Arc;
use std::thread;
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

    //Arc<str>
    let shared_string = Arc::new("Hello, concurrent world!");
    
    let mut handles = vec![];

    for i in 0..5 {
        let clone = Arc::clone(&shared_string);
        handles.push(thread::spawn(move || {
            println!("Thread {}: {}", i, clone);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    //Cow<'a, str>
    let bad_word: &str = "you are a badword";
    println!("sanitizing: {}", sanitize(bad_word));


}

fn sanitize(word: &str) -> Cow<str>{
    if word.contains("badword"){
        let sanitized: String = word.replace("badword", "******");
        return Cow::Owned(sanitized);
    }
    Cow::Borrowed(word)
}
