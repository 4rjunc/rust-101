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

    // s.clear(); // Error if uncommented, bcoz the word is referenced to s

    print!("First Word: {}", word);

    let hello = &s[..4];
    let hello1 = &s[0..4];

    println!("hello  :{}", hello);
    println!("hello1 :{}", hello1);

    let word = &[6..s.len()];
    println!("word: {:?}", &s[word]);
}
