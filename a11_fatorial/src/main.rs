// Solução clássica com for
fn fatorial_classico(n: i64) -> i64 {
    let mut fatorial: i64 = 1;

    for i in 1..=n {
        fatorial *= i;
    }
    fatorial
}

// Solução usando recursão, gasta muita memoria
fn fatorial_recursivo(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }

    n * fatorial_recursivo(n - 1)
}

// Solução usando iterador
fn fatorial_iterador(n: i64) -> i64 {
    (1..=n).product()
}

fn main() {
    let x: i64 = 4;
    println!("Fatorial classico de {}: {}", x, fatorial_classico(x));
    println!("Fatorial recursivo de {}: {}", x, fatorial_recursivo(x));
    println!("Fatorial iterador de {}: {}", x, fatorial_iterador(x));
}
