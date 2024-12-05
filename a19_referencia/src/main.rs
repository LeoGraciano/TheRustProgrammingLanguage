fn main() {
    let palavra = String::from("abacaxi");

    // Move: passa a propriedade do 'String'
    let len1 = calcula_tamanho_move(palavra.clone()); // clone uma string usa muito recurso, ideal é usar o Bowering para função do String
    println!("O Tamanho_1 da de '{}' é {}", palavra, len1);

    // Referencia: não passa a propriedade do String
    let len2 = calcula_tamanho_referencia(&palavra);
    println!("O tamanho_2 da de '{}' é {}", palavra, len2);

    // É a mesma coisa que na linguagem C?
    let x = 11;
    soma_900(&x);
    soma_900(&22);
}

// A propriedade do 'String' é recebida pela função
fn calcula_tamanho_move(s: String) -> usize {
    s.len()
}
// s fica inválido
// s tinha a propriedade do 'String', drop do 'String'

// Um empréstimo do 'String' é recebida pela função, e não a propriedade do 'String'
fn calcula_tamanho_referencia(s: &String) -> usize {
    s.len()
}
// s fica inválido
// s não tinha a propriedade do 'String', nenhum drop acontece

// Referência (empréstimo) não é a mesma coisa que endereço '&'¨com C
fn soma_900(ref_int: &i32) {
    let c_a = *ref_int + 900;
    let s_a = ref_int + 900;
    println!("com asterisco {}           sem asterisco {}", c_a, s_a);
}
