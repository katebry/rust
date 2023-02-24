fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    println!("{}", s);

    let mut hello_world = String::from("hello world");

    let r1 = &hello_world;
    let r2 = &hello_world;
    println!("r1: {}, r2: {}", r1, r2);

    let r3 = &mut hello_world;
    println!("r3: {}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
