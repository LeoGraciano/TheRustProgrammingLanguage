#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// unit-like structs, não tem campo nenhum
#[derive(Debug)]
struct AlwaysEqual;

struct User {
    active: bool,
    email: String,
    // email_recovery: &str, // User é dona de todos os seus campos
    username: String, // User não é dona de email2, requere 'lifetime' (Capitulo 10)
    sign_in_count: u64,
}
fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);

    let _novo: Color;
    // novo = origin; // Tipo diferentes
    _novo = black;

    println!("origin {:?}", origin);

    let unit_struct = AlwaysEqual;
    println!("unit_struct: {:?}", unit_struct);
}
