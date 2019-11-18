fn main() {
    let some_u8_value = Some(0u8);
    matched(some_u8_value);
    matched(Some(3u8));
}

fn matched(some_u8_value : core::option::Option) {
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}
