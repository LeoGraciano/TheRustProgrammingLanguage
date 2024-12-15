struct User {
    active: bool,
    email: String,
    username: String, // poder usar &str
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

    let user3 = build_user_com_slice("terceiro@example.com", "terceiro");

    println!("Email user3 {}", user3.email);

    // Forma não abreviada de inicialização
    let user4 = User {
        active: user1.active,
        email: String::from("quarto.doe@example.com"),
        username: user1.username, // o campo username:String de user 1 é movido para user4, user 1 fica inválido
        sign_in_count: user1.sign_in_count,
    };

    println!("Email user4 {}", user4.email);

    // let user5 = User {
    //     email: String::from("quinto@exemple.com"),
    //     ..user1  // o campo username:String de user1 é movido para user4, user1 fica inválido
    // };

    let user5 = User {
        email: String::from("quinto@exemple.com"),
        ..user4 //??  // o campo username:String de user4 é movido para user5, user1 fica inválido
    };
    println!("Email user5 {}", user5.email);
}

fn build_user_com_slice(email: &str, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 0,
    }
}

// Forma abreviada de inicialização
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}
