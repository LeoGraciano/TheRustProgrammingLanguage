fn main() {
    let s = String::from("hello");
    change1(&s);

    let mut x = String::from("hello");
    change2(&mut x);
}

// some_string é imutável
fn change1(some_string: &String) {
    // some_string.push_str(", world"); // imutável
    println!("{}", some_string); // imutável, apenas pintando o valor de referência
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}

/**/
// Varias referências imutáveis simultâneas para o mesmo valor é aceito
// Mas a cada momento pode haver APENAS UMA referência mutável para um valor
// A existência de uma referência mutável impede a existência de outras referências de qualquer tipo
fn nao_funciona() {
    let mut y = String::from("hello");

    // ryl deve sair do escopo antes dee ry2 ser criada
    let ry1 = &mut y; // y -> ry1 mut
    let ry2 = &mut y; // y -> ry2 mut
    println!("ry1 referencia {}", ry1);
    println!("ry2 referencia {}", ry2);

    let ry3 = &mut y; // no problem
    println!("ry3 referencia {}", ry3);
}

/**/
