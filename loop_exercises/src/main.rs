fn main() {
    let x = temperature_converter('f', 100.0);
    let y = temperature_converter('c', 35.0);
    println!("The value of f -> c: {x}");
    println!("The value of c -> f: {y}");

    for int in 0..15 {
        println!("fibonacci ({}) => {}", int, fibonacci_sequence(int));
    }
}

fn temperature_converter(unit: char, value: f32) -> f32 {
    if unit == 'f' {
        return (value - 32.0) * 0.5556 
    } else {
        return (value * 1.8) + 32.0
    }
}

fn fibonacci_sequence(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_sequence(n - 1) + fibonacci_sequence(n - 2)
    }
}