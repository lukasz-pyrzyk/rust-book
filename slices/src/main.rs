fn main() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("{word}");
}

fn first_word(s: &str) -> &str {
    let iter = s.as_bytes().iter();
    for (i, &byte) in iter.enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}