fn main() {
    let message: String = String::from("Hello Arjun");
    let word = first_word(&message);

    println!("First Word: {}", word);


    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..3];
    assert_eq!(slice, &[2,3]);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}
