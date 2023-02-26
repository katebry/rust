#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas
}

enum Coin {
    Penny, 
    Nickel,
    Dime, 
    Quarter(UsState),
}

fn main() {
    let coin_val = value_in_cents(Coin::Penny);
    let quarter_val = value_in_cents(Coin::Quarter(UsState::Arkansas));
    println!("{}", coin_val);
    println!("{}", quarter_val);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5, 
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
