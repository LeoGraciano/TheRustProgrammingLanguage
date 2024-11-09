fn main() {
    let mut number = 3;

    println!("\n   Usando while");
    while number != 0 {
        println!("Número: {}", number);
        number -= 1;
    }

    println!("\n   Usando for");
    let aaa = [10, 20, 30, 40, 50];

    for element in aaa {
        println!("Elemento: {}", element);
    }

    println!("\n   Usando ranger");
    for number in 1..=4 {
        println!("range: {}", number);
    }

    println!("\n   Usando range reverso");

    for number in (1..4).rev() {
        println!("reverso: {}", number);
    }
}
