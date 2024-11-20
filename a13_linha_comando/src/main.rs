use std::env;

fn main() {
    println!("\nTotal de elementos em env::ags é {}", env::args().len());

    println!("\nPercorre usando iterador:");
    let mut i = 0;
    for x in env::args() {
        println!("Argumento [{}] == {}", i, x);
        i += 1;
    }

    println!("\nPercorre usando iterador com indices:");
    for (i, x) in env::args().enumerate() {
        println!("Argumento [{}] == {}", i, x);
    }

    println!("\nCola tudo em um vector");
    let argumentos: Vec<String> = env::args().collect();

    println!("\nPercorre usando o vector");
    for i in 0..argumentos.len() {
        println!("Com vector fica Argumento [{}] == {}", i, argumentos[i]); // não é seguro usar indices em um vector
    }
    println!("Total de {} elementos no vector", argumentos.len());
}
