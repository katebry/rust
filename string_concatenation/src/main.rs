fn main() {
    let s1 = String::from("Hello ");
    let s2 = String::from(" world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    println!("{s3}");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let s = format!("{tic}-{tac}-{toe}");
    println!("{s}");

    for c in s.chars() {
        println!("{c}");
    }

    for b in s.bytes() {
        println!("{b}");
    }
}
