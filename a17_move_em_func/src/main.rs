fn main() {
    let s = String::from("hello");

    // Propriedade do 'String' entrega para a função
    recebe_ownership(s);
    // s não é mais Valido
    // println!("{}", s);

    let x = 5;
    recebe_copia(x); // x apenas copiado para a função

    println!("{}", x);
    // s não é dono do String, então  ele não é liberado da memoria nesse ponto
}

// um_string sai fora do escopo
// um_string fica inválido
// ele é dono do 'String'
// a memória do 'String' é liberada (drop)
fn recebe_ownership(um_string: String) {
    println!("{}", um_string);
}

fn recebe_copia(um_inteiro: i32) {
    println!("{}", um_inteiro);
}
