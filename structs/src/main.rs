fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Łukasz"),
        email: String::from("a@b.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("test@test.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("new@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("new@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let subject = AlwaysEqual;
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

struct AlwaysEqual;