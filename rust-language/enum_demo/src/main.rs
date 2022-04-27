#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    let ip = IpAddrKind::V6(String::from("192.168.1.1:IPV6"));

    route(ip);
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

// 给函数参数传递 enum
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4(v) => println!("{}", v),
        IpAddrKind::V6(v) => println!("{}", v),
    }
}
