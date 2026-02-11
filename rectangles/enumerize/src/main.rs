// enum IpAddrKind {
//     V4,
//     V6,
// }
// 
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

struct Ipv4Addr {} 
struct Ipv6Addr {} 
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", &self)
    }
}

enum MyOption<T> {
    None,
    Some(T),
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // 
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: MyOption<i32> = MyOption::None;

    // i8 + Option<i8> をしようとしているので、コンパイルーラーになる
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
}

// fn route(ip_kind: IpAddrKind) {}
