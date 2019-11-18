struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// explained in chapter 10, 
// structs with references
// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let email = String::from("someone@example.com");
    let username = String::from("someusername123");
    
    println!("email: {}, username: {}", email, username);

    let mut user1 = build_user(email, username);

    println!("user1.email before overwriting it {}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("user1.email after overwriting it {}", user1.email);


    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    print_user(1, user1);
    print_user(2, user2);

    let black = Color(0, 0, 0);
    println!("black: ({} {} {})", black.0, black.1, black.2);

    let origin = Point(0, 0, 0);
    println!("origin: ({} {} {})", origin.0, origin.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(name : u8 , user : User) {
    println!("user{}, username: {}, email: {}, active: {}, sign_in_count: {}", name, user.username, user.email, user.active, user.sign_in_count);
}
