fn main() {

    // this is immutable reference
    let message: String = String::from("This is RUST!");
    let len = calculate_len(&message); 
    println!("Message:{} has length: {}", message, len);

    // this is mutable reference
    let mut message: String = String::from("This Message is Mutable!");
    change_msg(&mut message); 
    println!("Mutable Message:{}", message);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    let str = dangle();
    println!("dangle: {}", s);
}

fn calculate_len(msg: &String) -> usize{ //msg is the reference to message
    msg.len()
}

fn change_msg(msg: &mut String) {
    msg.push_str("+This pushed string");
}

fn dangle() -> String{
    let message: String = String::from("GM");
    // &message returns a reference to data owned by current function. when function finishes
    // exceution the data is cleared which will cause dangling reference
    message
}
