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

fn main() {
    let penny_in_cents = value_in_cents(Coin::Penny);
    println!("penny_in_cents {}", penny_in_cents);

    let nickel_in_cents = value_in_cents(Coin::Nickel);
    println!("nickel_in_cents {}", nickel_in_cents);

    let dime_in_cents = value_in_cents(Coin::Dime);
    println!("dime_in_cents {}", dime_in_cents);

    let quarter_in_cents = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("quarter_in_cents {}", quarter_in_cents);

    let quarter_in_cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("quarter_in_cents {}", quarter_in_cents);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
