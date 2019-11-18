fn main() {
    // this moves the string to the function and returns it back
    let s1 = String::from("hello");

    let (s2, len) = calculate_length_and_return_tuple(s1);

    println!("The length of '{}' is {}.", s2, len);

    // this only references the string from calculate_length
    // and using &s1 we can just use the variable from that fn
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    // scoped mut, mut set on s twice using the block
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // _r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // dangling reference in this function, panic!s
    // dangle_reference();

    let s = no_dangle();
    println!("no_dangle returned {}", s);
}

fn calculate_length_and_return_tuple(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle_reference() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}