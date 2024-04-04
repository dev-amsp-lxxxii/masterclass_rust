pub fn imutaveis() {
    let x = 5;
    // x = 10; erro
    println!("x => {}", x);
}

pub fn mutaveis() {
    let mut x = 10;
    let y = x; // primitivo
    println!("x, y => {}, {}", x, y);

    x = 15;
    println!("x, y => {}, {}", x, y);
}

pub fn constante() {
    const Z: i32 = 20;
    println!("O valor de Z é: {}", Z);
}
