fn main() {
    // Cria a variável imutável s1, ela fica no stack pois tem tamanho fixo
    //
    // O tipo s1 é string slice ou uma referência para uma string no heap

    // s1 aponta para uma string no heap
    // string literals são imutáveis, fica fixo no código
    // "primeiro string literal" aponta para uma string no heap

    let s1 = "primeiro string literal";
    {
        let s2 = "segundo string literal";
        println!("Valor de s1 é {s1}");
        println!("Valor de s2 é {s2}");
    }
    println!("Valor de s1 é {s1}");
    // println!("Valor de s2 é {s2}"); // Isso causara um erro de compilação, devido ao escopo
}
