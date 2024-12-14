fn main() {
    let s = String::from("alo mundo azul");

    let word = first_word(&s);

    println!("Primeira palavra: {}", word);
}

// first_word retorna um 'string slice' &str
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
