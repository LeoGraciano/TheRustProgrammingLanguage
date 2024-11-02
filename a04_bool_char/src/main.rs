fn main() {
    let t = true;
    let f = false;

    let x = t && f;
    let y = t || !f;
    let z = 12 > 13;
    println!("boolean AND: {}, boolean OR: {}, boolean XOR: {}", x, y, z);

    let cc = 'z';
    let _c = 'z'; // sublinha elimina o warning
    let z: char = 'Z'; // char type is four bytes and represents a Unicode Scalar Value
    let x = t && (cc == z);
    println!("boolean AND: {x}, char: {z}");

    println!("boolean AND: {x}, char: {cc}");
}
