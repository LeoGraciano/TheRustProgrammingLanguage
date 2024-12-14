fn main() {
    let a = [11, 22, 33, 44, 45];
    let slice = &a[1..=3];

    for element in slice {
        println!("{element}");
    }
}
