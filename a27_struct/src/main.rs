struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("john.doe@example.com"),
        username: String::from("john_doe"),
        sign_in_count: 1,
    };
    println!("Email user1 {}", user1.email);

    user1.email = String::from("jane.doe@example.com");
    user1.sign_in_count += 1;
    println!("Email user1 {}", user1.email);

    let user2 = build_user(
        String::from("segundo.doe@example.com"),
        String::from("segundo"),
    );

    println!("Email user2 {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}
