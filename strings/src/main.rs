fn main() {
    let data = "initial contents"; // &str type =
    let s = data.to_string();// converts &str => String 
    println!("data: {}", data);


    //concatination 1
    // :: operator allows us to namespace this pariticular from function
    // under the String type. 
    let mut greeting = String::from("Hello");
    greeting.push_str(" World!");
    println!("greeting: {}", greeting);
    
    greeting.push_str(data);
    println!("greeting after data pushed: {}", greeting);

    //concatination 2
    let s1 = String::from("Aloo");
    let s2 = String::from(" Baji");
    //s1 is moved
    let s3 = s1 + &s2;
    println!("s3 = s1 + &s2, {} ", s3);
    // println!("s1: {}", s1); since s1 ownership is paaed but s2 reference is passed

    //concatination 3 
    let w1 = String::from("Mutta");
    let w2 = String::from("Pups");
    let w3 = format!("{}+{}", w1, w2);
    println!("w3: {}", w3);

    
    // some String operations
    let fruit = String::from("Jackfruit");
    println!("fruit's length: {:?}", fruit.len());
    println!("fruit in bytes: {:?}", fruit.as_bytes());
    println!("fruit in chars: {:?}", fruit.chars());

    // printing nth letter
    if !fruit.is_empty(){
        let first_letter = fruit.as_bytes()[0] as char;
        println!("The first character is: {}", first_letter);
    } else{
        println!("String is empty")
    }

    let s = String::from("Hello");
    if let Some(first_char) = s.chars().nth(0) {
        println!("The first character is: {}", first_char);
    } else {
        println!("The string is empty");
    }
    
    // heap data transfers ownership. other copies data
    // let a = String::from("Arjun");
    // let b = a;
    // println!("b:{}, a:{}",b, a);
}
