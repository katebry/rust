enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: i32 = v[2];
    println!("The third element is {third}");

    let second_third: Option<&i32> = v.get(2);
    match second_third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let second_v = vec![100, 31, 68]; 
    for i in &second_v {
        println!("{i}");
    }

    let mut third_v = vec![1, 2, 3, 10];
    for i in &mut third_v {
        *i += 10;
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
