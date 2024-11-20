fn main() {
    // Cria a variável mutável s3 ela fica no Stack pois tem tamanho fixo
    // let mut s3 =
    // s3 aponta para uma variável "Tipo String"
    //Como variável 'Tipo String' pode mudar de tamanho durante a execução ela é alocada no Heap
    //
    // let mut s3 = String::
    // O conteúdo inicial de variável ' Tipo String' vira de uma 'String literal' (tamanho inicial)
    // let mut s3 = String::from

    // 'String literal alo' é copiado para memória da variável 'Tipo String'

    let mut s3 = String::from("alo");
    println!("Valor de s3 é {s3}");

    //Podemos muda o conteúdo da variável ' Tipo String'
    //push_str() acrescenta um 'String literal' ao final da variável 'Tipo String'
    s3.push_str(", Mundo");

    // Mais memória foi alocada automaticamente para variável 'Tipo String'
    //
    // s3 não de tamanho pois ela é apontada para a variável 'Tipo String' no heap
    println!("Valor de s3 é {s3}");

    // s3 é dona (owns) a variável 'Tipo String' no heap
    // Como faço para liberar a memoria usada pela variável 'Tipo String' ???
    //                ******
    // A memória é liberada automaticamente quando termina o escopo do seu dono

    {
        let s4 = String::from("alo alo alo alo");
        println!("Valor de s4 é {s4}");
        // Tanto a memória de 's4' como do 'Tipo String' do qual ela é dona são liberadas automaticamente
        // Funções 'drop' que faz isso é chamada automaticamente pelo compilador.
    }
}
