struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("Hello, {}", user1.username);

    user1.username = String::from("anotherusername");

    println!("Hello, {}", user1.username);

    let user2 = build_user("test@example.com", "test")

    println!("Hello, {}", user2.username);

    let user3 = User {
        ...user2,
        email: String::from("another@example.com"),
    };

    let user4 = User {
        active: user1.active,
        username: user2.username,
        email: String::from("another@example.com"),
        sign_in_count: user2.sign_in_count,
    };

    println!("Hello, {}", user3.username);
    println!("Hello, {}", user4.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
