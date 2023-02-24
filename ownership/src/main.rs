fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hi");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}" , s1, s2);

    let str = String::from("hello");
    takes_ownership(str);

    let x = 5;
    makes_copy(x);

    println!("{}", x);
    // 'drop' has been called on str, and this memory freed
    // uncommenting the below line causes a compile error 👇🏻
    // println!("{}", str);

    let s3 = gives_ownership();
    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);

    println!("{} {}", s5, s3);

    let s6 = String::from("heyyyy");
    let (s7, len) = calculate_length(s6);

    println!("{} {}", s7, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}