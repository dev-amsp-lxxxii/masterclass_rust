fn main() {
    teste_a();
}

fn teste_a() {
    teste_b();
}

fn teste_b() {
    println!("{}", 123);
}

// Pilha: 1 - main(), 2 - teste_a(), 3 - teste_b()
// Stack - LiFo
// Todos os dados que tem tamanho fixo serão colocados na stack
