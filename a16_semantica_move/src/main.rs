fn main() {
    // Semântica 'copy'

    let x = 5;
    let y = x; // valor 5 é copiado para o y

    println!("y = {y}");
    println!("x = {x}");

    // Semântica copy disponível quando o tipo implementa o 'trail' Copy
    // Inteiros
    // Booleanos
    // Char
    // Pronto flutuante
    // Tuplas (quando os seus tipos implementam o 'trail' Copy)
    // (i32, bool) sim
    // (i32, String) nao

    // Semântica 'move'
    let s1 = String::from("hello");
    let s2 = s1; // s1 eh movida para s2

    println!("s1 = {s2}");
    // println!("s2 = {s1}"); // s1 não é valida
}
