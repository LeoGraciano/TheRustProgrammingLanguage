fn main() {
    let number = 3;

    if number < 5 {
        println!("condição verdadeira");
    } else {
        println!("condição falsa");
    }

    // Cascata de if-else
    if number % 4 == 0 {
        println!("O número é divisível de 4");
    } else if number % 3 == 0 {
        println!("O número é divisível de 3");
    } else if number % 2 == 0 {
        println!("O número é divisível de 2");
    } else {
        println!("O número não é divisível de 4, 3 ou 2");
    }

    // pode user como expressão
    let outro_numero = if number == 0 { 0 } else { 99 };
    println!("Outro número: {}", outro_numero);
}
