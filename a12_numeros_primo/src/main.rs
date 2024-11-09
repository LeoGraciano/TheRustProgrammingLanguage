fn numero_primo_while(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    let limite = (n as f64).sqrt() as u32;
    let mut divisor = 2;
    while divisor <= limite {
        if n % divisor == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}

fn numero_primo_for(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    let limite = (n as f64).sqrt() as u32;
    for divisor in 2..=limite {
        if n % divisor == 0 {
            return false;
        }
    }
    true
}

fn numero_primo_for_v2(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    if n % 3 == 0 {
        return false;
    }
    let limite = (n as f64).sqrt() as u32;
    for divisor in (5..=limite).step_by(2) {
        if n % divisor == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("\nnumero_primo_while({}) -> {}", 1, numero_primo_while(1));
    println!("numero_primo_while({}) -> {}", 2, numero_primo_while(2));
    println!("numero_primo_while({}) -> {}", 3, numero_primo_while(3));
    println!("numero_primo_while({}) -> {}", 8, numero_primo_while(8));
    println!("numero_primo_while({}) -> {}", 97, numero_primo_while(97));

    println!("\nnumero_primo_for({}) -> {}", 1, numero_primo_for(1));
    println!("numero_primo_for({}) -> {}", 2, numero_primo_for(2));
    println!("numero_primo_for({}) -> {}", 3, numero_primo_for(3));
    println!("numero_primo_for({}) -> {}", 8, numero_primo_for(8));
    println!("numero_primo_for({}) -> {}", 97, numero_primo_for(97));

    println!("\nnumero_primo_for_v2({}) -> {}", 1, numero_primo_for_v2(1));
    println!("numero_primo_for_v2({}) -> {}", 2, numero_primo_for_v2(2));
    println!("numero_primo_for_v2({}) -> {}", 3, numero_primo_for_v2(3));
    println!("numero_primo_for_v2({}) -> {}", 8, numero_primo_for_v2(8));
    println!("numero_primo_for_v2({}) -> {}", 97, numero_primo_for_v2(97));
}
