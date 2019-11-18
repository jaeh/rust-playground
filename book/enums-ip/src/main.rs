enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}


enum IpAddrVariants {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home_enum = IpAddrEnum::V4(String::from("127.0.0.1"));

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let loopback_enum = IpAddrEnum::V6(String::from("::1"));


    let home = IpAddrVariants::V4(127, 0, 0, 1);

    let loopback = IpAddrVariants::V6(String::from("::1"));
}
