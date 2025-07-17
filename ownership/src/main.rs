fn main() {
    let s = String::from("Hello World");

    take_ownership(s);

    let x = 5;
    make_a_copy(x);
    println!("x:{}", x);
    // this print is invalid since the ownership of s is tranfered to the fn.
    //println!("s: {}", s);

    //function returning multiple values
    let message = String::from("Im Batman");
    let (msg, len) = calculate_msg(message);
    println!("message:{} has length:{}", msg, len);
}

fn calculate_msg(msg: String) -> (String, usize) {
    let length = msg.len();

    (msg, length)
}
fn take_ownership(str: String) {
    println!("str: {}", str);
}

fn make_a_copy(num: i32) {
    println!("num:{}", num);
}
