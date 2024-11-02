fn main() {
    let tup1: (i32, f64, bool) = (500, 6.4, true);
    println!("{:?}", tup1);

    let tup2 = (100, 8.2, false);
    println!("{:?}", tup2);

    // desestruturação (destructuring) quebra a tupla em suas partes
    let (x, y, z) = tup1;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Tuplas podem ser indexadas
    println!("Indexando tupla: a={}, b={}, c={}", tup2.0, tup2.1, tup2.2);

    // Uma tupla vaia é chamada unit, representa uma valor vazio
    let unit_tuple: () = ();
    println!("Tupla vazia: {:?}", unit_tuple);
    println!("Tupla vazia: {:?}", ());
}
