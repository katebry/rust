fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    let last = last_word(&s);

    println!("first word: {word}");
    println!("last word: {last}");

    s.clear();

    // you can also slice an array 👇🏻
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn last_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (_, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[6..];
        }
    }
    &s[..]
}