#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Nickel,
    Quarter(UsState),
}

fn main() {
    let nickel_in_cents = value_in_cents(Coin::Nickel);
    println!("nickel_in_cents {}", nickel_in_cents);
    
    let quarter_in_cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("quarter_in_cents {}", quarter_in_cents);

    let quarter_in_cents_if_let = value_in_cents_if_let(Coin::Quarter(UsState::Alabama));
    println!("quarter_in_cents_if_let {}", quarter_in_cents_if_let);
}

fn value_in_cents(coin: Coin) -> u8 {
    let count = 0;
    match coin {
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            count
        },
        _ => count + 1,
    }
}

fn value_in_cents_if_let(coin: Coin) -> u8 {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    count
}