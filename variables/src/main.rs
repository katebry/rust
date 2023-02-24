fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let spaces = "    ";
    println!("The value of spaces is: {spaces}");

    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");
    println!("The value of remainder is: {remainder}");

    let ghost = '👻';
    println!("The value of ghost is: {ghost}");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of A is: {a}, the value of B is: {b}, the value of C is: {c}");
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");

    // arrays
    let array_of_five_vals: [i32; 5] = [1, 2, 3, 4, 5];
    let array_of_threes = [3; 5];

    let first = array_of_five_vals[0];
    let last = array_of_threes[4];

    println!("The value of first is: {first}, and the value of last is: {last}");
}
