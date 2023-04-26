struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@ecample.com"),
        sign_in_count:1,
    };

    println!("{}", user1.username);

    let mut user2 = User {
        active: true,
        username: String::from("thiswillnotshow"),
        email: String::from("random@yeap.com"),
        sign_in_count: 2,
    };

    user2.username = String::from("thisisthename");
    println!("{}", user2.username);

    let user3 = build_user(String::from("test123@email.com"), String::from("testuser"));
    println!("{}, {}", user3.username, user3.email);

    let user4 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}", user4.username);

    let black = Color(0, 0, 0);
    println!("{}", black.0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
