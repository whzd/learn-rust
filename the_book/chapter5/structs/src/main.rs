struct User {
    _active: bool,
    username: String,
    email: String,
    _sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        _active: true,
        username,
        email,
        _sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someunsername123"),
        _active: true,
        _sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(user1.email, user1.username);

    let _user3 = User {
        email: String::from("onemore@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let Color(_b1, _b2, _b3) = black;

    let _subject = AlwaysEqual;
}
