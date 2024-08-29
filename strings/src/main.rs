fn main() {
    let data = "initial contents"; // &str type =
    let s = data.to_string();// converts &str => String 
    println!("data: {}", data);


    //concatination 1
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
}
