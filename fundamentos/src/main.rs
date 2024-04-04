fn main() {
    // Espaços diferentes na Stack
    let lista_a = [1, 2, 3, 4, 5];
    let lista_b = lista_a; // copy

    // Neste caso tamanhos fixo, então está sendo
    // trabalhado na stack
    println!("{:?}", lista_a);
    println!("{:?}", lista_b);
}
