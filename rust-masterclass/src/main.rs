mod utils;

use utils::terminal::{esperar_enter, exibir_menu};

fn main() {
    let itens = ["Fundamentos", "Tipos", "Controle", "Funções", "Ownership"];

    exibir_menu("Principal", &itens, true);
    esperar_enter();
    /*
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    esperar_enter();
    limpar_tela();
    println!("Hello, world!");
    */
}
