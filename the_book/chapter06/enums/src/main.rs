enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Function called!");
    }
}

fn main() {
    // Emum examples
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    //Option enum
    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;
}
