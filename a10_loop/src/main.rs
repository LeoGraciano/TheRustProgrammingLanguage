fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i % 2 == 0 {
            continue;
        }
        println!("Número: {}", i);
        if i >= 10 {
            break;
        }
    }

    let result = loop {
        i = 100;
        if i >= 100 {
            break i * 2;
        }
    };
    println!("Resultado: {}", result);

    println!("\nLabels em loops");
    let mut contagem = 0;

    'meu_externo: loop {
        println!("Contagem externa: {}", contagem);
        contagem += 1;
        let mut faltam = 100;

        loop {
            println!("faltam: {}", faltam);
            if faltam == 97 {
                break; // break do loop interno
            }
            if contagem == 2 {
                break 'meu_externo;
            }
            faltam -= 1;
        }
        println!("incrementa contagem");
        contagem += 1;
    }
    println!("Contagem final: {}", contagem);
}
