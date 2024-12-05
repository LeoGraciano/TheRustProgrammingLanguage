fn main() {
    let s1 = devolve_ownership(); // Valor de retorno é movido para S1

    let mut s2 = String::from("hello");
    let s3 = recebe_e_devolve_ownership(s2); // s2 é movido para a função, s2 fica invalido
                                             // o valor de retorno da função é movido para s3

    s2 = String::from("world");
    println!("s1:{} s2:{} s3:{}", s1, s2, s3); // o println! ele passa como referência automaticamente

    println!("s1:{} s2:{} s3:{}", &s1, &s2, &s3); // o println! ele pega reference do borrowing

    // s1 fica inválido, sua propriedade sofre um drop
    // s2 fica inválido, mas não é dono de nada
    // s3 fica inválido, sua propriedade sofre um drop
}
fn devolve_ownership() -> String {
    let algo = String::from("aaaa");
    algo // Conteúdo de 'algo' retorno para a função chamadora, semântica 'move' pois é um 'String'
         // Variável 'algo' fica inválida
}

fn recebe_e_devolve_ownership(um_string: String) -> String {
    println!("{}", um_string);
    um_string // Conteúdo de 'um_string' é retornado para a função chamadora semântica 'move' pois é um 'String'
}
