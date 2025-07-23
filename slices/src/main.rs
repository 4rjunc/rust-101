fn first_word(s: &str) -> &str {
    let s_array = s.as_bytes();

    for (i, &item) in s_array.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return s;
}

fn main() {
    let s = String::from("Hello, world!");

    let word = first_word(&s);

    print!("First Word: {}", word);
}
