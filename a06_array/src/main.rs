fn main() {
    let aa = [1, 2, 3, 4, 5];
    let meses = [
        "Janeiro",
        "Fevereiro",
        "Março",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];
    println!("messes: {:?}", meses);

    let bb: [i32; 5] = [1, 2, 3, 4, 5];

    let cc = [3; 5];
    let dd = [3, 5];

    println!("Array: {:?}", aa);
    println!("Vetor: {:?}", bb);
    println!("Array de inteiros com 3 elementos: {:?}", cc);
    println!("Array de inteiros com 5 elementos: {:?}", dd);
}
