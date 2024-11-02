const UM_HORA_EM_SEGUNDOS: u32 = 1 * 60 * 60; // Definindo uma constante em Rust, como 'UM_HORA_EM_SEGUNDOS' é uma constante, não pode ser alterado posteriormente

fn main() {
    println!("VARIAÇÃO DE VARIÁVEIS");
    let x = 5;
    println!("O valor de x é: {}", x);

    // x = 6; // Isso causará um erro de compilação, pois 'x' é uma imutável

    let mut x = 666; // Isso é permitido pois 'x' sendo declarado novamente
    println!("Novo valor de x agora é: {}", x);

    let mut y = 10;
    println!("O valor de y é: {}", y);
    y = 6; // Isso é permitido pois 'y' é mutável
    println!("Novo valor de y agora é: {}", y);

    println!("CONSTANTE");
    x = UM_HORA_EM_SEGUNDOS; // Isso é permitido pois 'x' é mutável
    println!("Novo valor de x agora é: {}", x);
    const UM_HORA_EM_SEGUNDOS: u32 = 1 * 600 * 600; // ESCOPO INTERNO DA FUNÇÃO e prevalece o escopo global

    let mut x = 5;
    println!("O valor de x é: {}", x);

    x = UM_HORA_EM_SEGUNDOS; // Isso é permitido pois 'x' é mutável
    println!("Novo valor de x agora é: {}", x);

    println!("BLOCOS");
    const X: i32 = 5;
    let y = 6;
    let mut z = 7;
    z = z + 1;
    println!("No inicio os valores são: X = {}, y = {}, z = {}", X, y, z);
    {
        const X: i32 = 555;
        let y = 666;
        let mut z = 777;
        z = z + 1;
        println!(
            "Dentro do bloco interno, os novos valores são: X = {}, y = {}, z = {}",
            X, y, z
        );
    }

    println!(
        "Volta valores foram depois do bloco interno: X = {}, y = {}, z = {}",
        X, y, z
    );
}
