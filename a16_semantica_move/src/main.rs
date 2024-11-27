/*
Copy: "Types whose values can be duplicated simply by copying bits."
    É uma copia burra de bit a bit
    É mais rápida
    Acionado pela atribuição '='
    Tendo Copy automaticamente tem Clone

Clone: "A common trait for the ability to explicitly duplicate an object."
    É uma duplicação inteligente de todos os componentes e subcomponentes
    É mais lento que Copy
    Acionando pelo método 'clone()'
    Pode ter Clone mas não ter Copy
*/

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
    let s2 = s1; // s1 é movida para s2

    println!("s1 = {s2}");
    // println!("s2 = {s1}"); // s1 não é valida

    // ainda é possível fazer um clone
    let s3 = s2.clone();
    println!("s2 = {s3}");
    println!("s2 = {s2}");
}
