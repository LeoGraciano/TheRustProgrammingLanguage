fn main() {
    // semântica 'copy'
    let x = 5;
    let y = x; // valor 5 é copiado para o y

    println!("y = {}", y);
    println!("x = {}", x);

    // Semântica copy disponível quando o tipo implementa o 'trait' Copy
    // Inteiros
    // Booleanos
    // Ponto flutuante
    // Caracteres
    // Tuplas e Array apenas com tipo que suportam Copy
    //      (i32, bool) sim
    //      (i32, String) não

    // semântica 'move'
    let s1 = String::from("Hello");
    let s2 = s1; // Valor 'Tipo String' é movido para s2

    println!("\ns2 = {}", s2);
    // println!("s1 = {}", s1); // Erro: s1 não é mais válido

    // Ainda é possível fazer um clone
    let s3 = String::from("Hello world to clone");
    let s4 = s3.clone();
    println!("\ns4 = {}", s4);
    println!("s3 = {}", s3);

    // Ainda é possível só emprestar  (borrowing) imutável
    let s5 = String::from("imutável");
    let s6 = &s5; // s6 é um apontador para o s5

    println!("\ns5 = {s5}");
    println!("s6 = {s6}");

    // Pode emprestar se é mutável ?

    let mut s7 = String::from("mutável");
    let s8 = &s7;

    println!("\ns8 = {s8}");
    println!("s7 = {s7}");

    s7.push_str("000");
    println!("\ns7 = {s7}");
}
