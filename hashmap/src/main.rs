use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let other_team = String::from("Yellow");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    let other_score = scores.get(&other_team).copied().unwrap_or(0);

    println!("Blue team score: {score}! Yellow team score: {other_score}!");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favourite colour");
    let field_value = String::from("Green");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    for (key, value) in &map {
        println!("{key}: {value}");
    };

    // overwriting a value
    let mut total = HashMap::new();
    total.insert(String::from("Green"), 10);
    total.insert(String::from("Green"), 30);

    println!("{:?}", total);

    // adding a key or value only if a key isn't present
    let mut new_total = HashMap::new();
    new_total.insert(String::from("Pink"), 10);

    new_total.entry(String::from("Yellow")).or_insert(55);
    new_total.entry(String::from("Pink")).or_insert(40);

    println!("{:?}", new_total);

    // updating a value based on the old value
    let text = "hello world what a wonderful world";
    let mut hash_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = hash_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", hash_map);
}
