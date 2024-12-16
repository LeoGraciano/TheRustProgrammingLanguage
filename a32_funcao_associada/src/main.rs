#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Função associada, retorna um novo Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // Método pois tem &self
    fn area(&self) -> u32 {
        // self: &Self
        // poderia ser '&mut self' ou 'self'
        self.width * self.height
    }
}
impl Rectangle {
    // Método pois tem &self
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let sq = Rectangle::square(3);

    println!(
        "A área do retângulo rect1 é {} pixels quadrados",
        rect1.area()
    );
    println!("A área do retângulo sq é {} pixels quadrados", sq.area());
}
