fn main() {
    println!("Hello, world!");

    another_function(5, 23);

    let (x, y) = run_expression();
    println!("run_expression returned x: {} y: {}", x, y);

    let f = five();
    let t = twentythree();

    println!("returned from five(): {} returned from twenty_three() {}", f, t);

    let f = plus_one(f);
    println!("plus_one(five) returned {}", f);
    
    let t = plus_one(t);
    println!("plus_one(twenty_three) returned {}", t);
}

fn another_function(x: i32, y: i32) {
    println!("Another function. x: {} y: {}", x, y);
}

fn run_expression() -> (i32, i32) {
    let x = 5;

    let result = {
        let y = 3;
        (x, y)
    };

    result
}

fn five() -> i32 {
    5
}

fn twentythree() -> i32 {
    23
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
