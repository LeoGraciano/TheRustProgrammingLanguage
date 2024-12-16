#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // self: &Self
        // poderia ser '&mut self' ou 'self'
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("A área do retângulo é {} pixels quarados", rect1.area());

    println!("Consegue rect1 conter rect2? {}", rect1.can_hold(&rect2));

    //Automatic referencing and deferencing
    println!("Consegue rect1 conter rect3? {}", rect1.can_hold(&rect3));
    // println!("Consegue rect1 conter rect3?", rect1.can_hold(rect3));
    println!("Consegue rect1 conter rect3? {}", rect1.can_hold(&rect3));
}
