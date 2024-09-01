fn main() {
    let s1: &str = "Hello World 🦀";
    let s2: String = String::from("Hello World! 🦀");
    let s3: String = "Hello World".to_string();
    let s4: String = "Hello World".to_owned();
    let s5: &str = &s4[3..=6]; // points the part of the string from 3rd character to 6th character

    println!("s5: {}", s5);

    let mut name: String = String::from("Arjun");
    //pushing characters
    name.push_str(" Mundamani");
    println!("name: {}", name);
    // replace characters
    name.replace_range(5.., " C"); // replace from 5th character till end 
    println!("name: {}", name);
    //concat.
    let fruit_1: String = String::from("Jack");
    let fruit_2: String = String::from(" Fruit");
    let fruit_name: String = fruit_1 + &fruit_2; // method 1
    println!("fruit_name: {}", fruit_name);

    let word1: String = String::from("The");
    let word2: String = String::from("Monkey");
    let word3: String = String::from("Is");
    let word4: String = String::from("Crying");
    // method 2 less efficent than + operator
    let sentence: String = format!("{}-{}-{}-{}", word1, word2, word3, word4); 
    println!("sentence: {}", sentence);

    let word5: String = ["i", "am", "a", "good", "boy"].concat();
    let word6: &str = concat!("Hi","How", "Are", "you?");

    let word7: String = String::from("Nice");
    let sentence2: String = word7 + "Car"; // String must be the first

    // indexing into the string
    let s1: &str = "🦀the🦀🦀🦀";
    let s2: &str = &s1[0..4];
    println!("s2: {}", s2);

    //iterating in a string
    //iterating bytes 
    for b in "MUNDAMANI".bytes() {
        println!("{}",b);
    }
    //iterating chars 
    for c in "ARJUN".chars(){
        println!("{}", c);
    }
    
    let fn_word = "gm";
    println!("my_function: {}", my_function(fn_word));
    println!("my_function(&s2):{}", my_function(&s2));
}

fn my_function(a: &str) -> String{
    return format!("{} !",a);
}
