fn main() {
    let my_string = String::from("alo mundo azul");

    // Parâmetros pode ser slice
    let word = first_word(&my_string[0..6]);
    println!("{word}");

    let word = first_word(&my_string[..]);
    println!("{word}");

    // Parâmetros pode ser referência para String
    let word = first_word(&my_string);
    println!("{word}");

    let my_string_literal = "hello world";

    // Parâmetro pode ser slice de 'String Literal'
    let word = first_word(&my_string_literal[0..6]);
    println!("{word}");

    let word = first_word(&my_string_literal[..]);
    println!("{word}");

    // Parâmetro pode ser uma 'String Literal', pois ele equivale a um '&str'
    let word = first_word(my_string_literal);
    println!("{word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
