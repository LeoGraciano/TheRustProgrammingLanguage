use std::io;

fn tamanho_palavra_v1() {
    println!("\n\ntamanho_palavra_v1()");

    //vec<T> tem dados contíguos, pode acessar com array, pode varia o tamanho
    let mut palavras = Vec::new();

    loop {
        println!("[V1] Digite uma palavra ou somente enter para terminar");
        let mut linha = String::new();
        io::stdin()
            .read_line(&mut linha)
            .expect("Error ao ler o teclado");
        linha = linha.trim().to_string();
        //linha = linha.trim(); // trim() retorna slice
        if linha.len() == 0 {
            break;
        } else {
            println!("lido: {}", linha); // println! não move (não invalida) 'linha'
            palavras.push(linha);
            // println!("linha: {}", linha); // 'linha' foi movida (invalida) por Vec::push()
        }
    }

    for p in palavras {
        println!("{p}");
    }
}

// Calcula tamanho mínimo, médio e máximo de uma lista de palavras
// Pode ter várias palavras por linha
fn tamanho_palavra_v2() {
    println!("[V2] Digite várias palavras ou somente enter para terminar");
    let mut lista_palavras = Vec::new();
    // let mut palavras: vec::new(); // já deixa indicado que

    loop {
        let mut linha = String::new();
        io::stdin()
            .read_line(&mut linha)
            .expect("Error ao ler o teclado");
        linha = linha.trim().to_string();
        if linha.len() == 0 {
            break;
        } else {
            // let palavras  = linha.split(" "); // 'split' retorna sobre slices da linha
            let palavras = linha.split_whitespace(); // desconsidera vária espaços entre palavras

            for p in palavras {
                //lista_palavras.push(p.trim()); // 'p' e 'palavras' destruidos antes de 'lista_palavras'
                //lista_palavras.push (p.trim().clone()); // clone de slice é slice
                lista_palavras.push(p.trim().to_string()); // cria String para colocar em 'lista_palavras'
            }
        }
    }
    println!("Foram digitadas {} palavras\n", lista_palavras.len());

    let mut minimo = 999999;
    let mut maximo = 0;
    let mut total = 0;

    for p in lista_palavras.iter() {
        let tam = p.chars().count();
        total += tam;
        if tam < minimo {
            minimo = tam;
        }
        if tam > maximo {
            maximo = tam;
        }
        println!("{p}")
    }
    if lista_palavras.len() > 0 {
        println!("\nTamanho mínimo: {}", minimo);
        println!("Tamanho máximo: {}", maximo);
        println!("Tamanho médio: {}", total / lista_palavras.len());
    }
}
fn main() {
    tamanho_palavra_v1();
    tamanho_palavra_v2();
}
