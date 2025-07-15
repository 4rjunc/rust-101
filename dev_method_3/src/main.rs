use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //  let x = 5;
    let mut x = 5; // default i32
    println!("Value of x: {}", x);

    x = 6;
    println!("Value of x: {}", x);

    let y = 3.0; // f64
    let z: f32 = 4.0;

    let t = true;
    let f: bool = false;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundered = tup.0;

    let a = [11, 22, 33, 44, 55];
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
    let index: usize = option.trim().parse().expect("Not a number");

    if index >= a.len() {
        return Err("You guessed out of scope".into());
    }

    println!("Options value: {}", a[index]);

    let a = 10;
    let b = sum(a);
    println!("Function return b: {}", b);
    println!("Function return a: {}", a);

    let mut name = String::from("DADHU");
    let len = len(&mut name);
    println!("len: {}", len);

    Ok(())
}

fn len(name: &mut String) -> String {
    //name.len()

    name.split_off(2)
}

fn sum(x: u32) -> u32 {
    x
}
