use std::cmp::PartialOrd;

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
    let integer_and_float = Point {x: 20, y: 3.4};

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}