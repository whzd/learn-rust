enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKind2 {
    V4(String),
    V6(String),
}

enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind3 {
   fn call(&self) {
       println!("Called!");
   } 
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddrKind2::V4(String::from("127.0.0.1"));

    let loopback2 = IpAddrKind2::V6(String::from("::1"));

    let home3 = IpAddrKind3::V4(127, 0, 0, 1);

    let loopback3 = IpAddrKind2::V6(String::from("::1"));
    home3.call();
}
