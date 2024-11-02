fn main() {
    println!("SOBREAMENTO");
    let x = 5;
    println!("O valor de x é: {}", x);
    let x = x + 1;
    println!("Novo valor de x agora é: {}", x);

    {
        let x = x * 2;
        println!("Novo valor de x dentro do bloco interno é: {}", x);
    }
    println!("Novo valor de x fora do bloco interno é: {}", x);

    let spaces = "    ";
    let spaces = spaces.len(); // let cria uma nova variável com novo tipo
    println!("Número de space: {}", spaces);

    let mut space2 = "    ";
    println!("Valor inicial de space2: {space2}");
    space2 = "qwerty";
    println!("Novo valor de space2: {space2}");
}
