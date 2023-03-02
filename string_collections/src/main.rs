fn main() {
    let mut s = String::new();
    let mut foo = String::from("foo");
    let str_from = String::from("initial contents");

    let data = "initial contents";

    let s = data.to_string();

    foo.push_str("bar");

    println!("{s}");
    println!("this is the same as calling .to_string: {str_from}");
    println!("the value of foo is: {foo}");

    foo.push('!');
    println!("the value of foo is: {foo}");


    // the method also works on a literal directly
    let s2 = "initial contents".to_string();

    println!("{s2}");
}
