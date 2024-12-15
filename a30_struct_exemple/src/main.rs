// #[derive(Debug)] antes da struct torna 'Debug' disponível
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scala = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scala),
        height: 50,
    };

    println!("A área do retângulo é {} pixels quadrados", area(&rect1));

    // println!("rect1 é {}", react); // Rectangle precisa ter a característica (trai) 'std::fmt::Display'
    println!("rect1 é {:?}", rect1); // Rectangle precisa ter a característica (tail) 'std::fmt::Debug'
    println!("rect1 é {:#?}", rect1); // Rectangle precisa ter a característica (tail) 'std::fmt::Debug'

    println!("\n\nchamando area2 ...\n");
    area2(&rect1);
    let r2 = area2(&rect1);
    println!("r2 vale {r2}");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area2(rectangle: &Rectangle) -> u32 {
    dbg!(&rectangle); // Útil para depurar
    dbg!(rectangle.width * rectangle.height) // Útil para depurar, aqui é o retorno da função
}
