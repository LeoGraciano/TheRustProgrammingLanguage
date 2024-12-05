fn main() {
    // Slices são referências para uma sequência de elementos
    // UM slice é um tipo de referência

    let s = String::from("Hello, world!");

    let s1 = &s[0..5]; // Tipo de &str ou 'string slice'
    let s2 = &s[6..11];
    let s3 = &s[..2];
    let s4 = &s[3..];

    let slit = "Hello, world!"; // slit é to tipo &str ou 'string slice'

    println!(
        "s1: {}, s2: {}, s3: {}, s4: {}, slit: {}",
        s1, s2, s3, s4, slit
    );
}
