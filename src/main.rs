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
            println!("State of the coin: {:?}", state);
            25
        }
    }
}

fn main() {
    println!(
        "The Quarter: {:?} \n",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    println!("The Penny: {:?} \n", value_in_cents(Coin::Penny));
}
