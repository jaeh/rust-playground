fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    println!("some number {:?}", some_number);

    println!("some_number.is_some() {}", some_number.is_some());
    println!("some_number.is_none() {}", some_number.is_none());

    println!("some_string {:?}", some_string);
    println!("some_string.is_some() {}", some_string.is_some());
    println!("some_string.is_none() {}", some_string.is_none());

    let absent_number: Option<i32> = None;
    println!("absent number {:?}", absent_number);

    println!("absent_number.is_some() {}", absent_number.is_some());
    println!("absent_number.is_none() {}", absent_number.is_none());

    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {}
    }
    assert_eq!(x, Some(42));

    let x = Some("value");
    assert_eq!(x.expect("the world is ending"), "value");

    let _x: Option<&str> = None;
    // panics with `the world is ending`
    // x.expect("the world is ending");

    let k = 10;

    let some_unwrap_or_else = Some(4).unwrap_or_else(|| 2 * k);
    println!("Some(4).unwrap_or_else() returns: {}", some_unwrap_or_else);

    let none_unwrap_or_else = None.unwrap_or_else(|| 2 * k);
    println!("None.unwrap_or_else() returns: {}", none_unwrap_or_else);

    // x.map_or returns a static default value
    let x = Some("foo");
    println!("x.map_or with Some {}", x.map_or(42, |v| v.len()));

    let x: Option<&str> = None;
    println!("x.map_or with None {}", x.map_or(42, |v| v.len()));

    // x.map_or_else returns a computed value
    let k = 21;

    let x = Some("foo");
    println!("x.map_or_else Some returns {}", x.map_or_else(|| 2 * k, |v| v.len()));

    let x: Option<&str> = None;
    println!("x.map_or_else None returns {}", x.map_or_else(|| 2 * k, |v| v.len()));

}
