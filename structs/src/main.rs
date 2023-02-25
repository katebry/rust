struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("test"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
    };

    println!("active: {}, username: {}, email: {}, sign_in_count: {}", user1.active, user1.username, user1.email, user1.sign_in_count);

    user1.username = String::from("not_a_test");

    println!("{}", user1.username);

    let email = String::from("me@test.com");
    let username = String::from("me");

    let user2 = build_user(email, username);

    println!("email: {}, username: {}", user2.email, user2.username);

    let user3 = User {
        active: user1.active,
        username: user2.username,
        email: user1.email,
        sign_in_count: 7,
    };

    println!("email: {}, sign_in_count: {}", user3.email, user3.sign_in_count);

    let user4 = User {
        email: String::from("thisismyemail@test.com"),
        ..user3
    };

    println!("email: {}", user4.email);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}