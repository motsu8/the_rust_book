#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
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

fn main() {
    let c1 = Coin::Penny;
    let res = value_in_cents(c1);
    println!("Coin value is: {}", res);

    let c2 = Coin::Quarter(UsState::Alaska);
    println!("Coin value is: {}", value_in_cents(c2));
}
