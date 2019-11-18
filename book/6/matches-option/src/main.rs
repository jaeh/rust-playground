
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five {:#?}", five);
    println!("six {:#?}", six);
    println!("none {:#?}", none);

    let some_u8_value = 0u8;

    matching(some_u8_value);
    matching(1u8);
    matching(2u8);
    matching(3u8);
    matching(5u8);
    matching(7u8);
    matching(55u8);
    matching(255u8);
}

fn matching(some_u8_value: u8) {
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("_ unhandled number {}", some_u8_value),
    };
}