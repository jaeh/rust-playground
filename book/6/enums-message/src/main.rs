// this enum is equivalent to the four structs below
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct _QuitMessage; // unit struct
struct _MoveMessage {
    x: i32,
    y: i32,
}
struct _WriteMessage(String); // tuple struct
struct _ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}