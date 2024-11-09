fn outra_funcao() {
    println!("Outra função");
}
fn outra_funcao_com_paramentros(x: i32) {
    println!("Outra Função recebeu {x}")
}

fn print_labeled_measurement(valor: f64, unidade: char) {
    println!("Valor: {:.2} {}m", valor, unidade);
}

fn soma(x: i32, y: i32) -> i32 {
    x + y // Função que retorna a soma dos dois valores, não precisa colocar o ;
}

fn soma_return(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    outra_funcao();
    outra_funcao_com_paramentros(10);
    print_labeled_measurement(10.5, 'm');
    println!("Soma: {}", soma(3, 7));
    println!("Soma: {}", soma_return(3, 9));
    let y = {
        let x = 3;
        x + 1
    };
    println!("Valor de y: {:?}", y);
}
