fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    println!("operations");
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let added = sum + product + remainder;

    let floated = difference + quotient;
    println!("operations finished {} {}", added, floated);


    println!("booleans");
    
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("true {}, false {}", t, f);

    println!("chars");

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("chars finished {} {} {}", c, z, heart_eyed_cat);

    println!("tuples");
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y from tuple is: {}", y);

    println!("tuples with numeric key access");
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("numeric tuple result {}, {}, {}", five_hundred, six_point_four, one);

    println!("arrays");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months {}", months.join(" "));

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("a {} {}", first, second);

    println!("invalid array index that panics");
    // let a = [1, 2, 3, 4, 5];
    // let index = 10;

    // let element = a[index];

    // println!("The value of element is: {}", element);
}
