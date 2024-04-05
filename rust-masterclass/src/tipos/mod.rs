mod basicos;
mod custom;
mod sequencias;

use crate::utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

pub fn executar() {
    loop {
        let itens = [
            "Básicos",
            "Sequências",
            "Custom - Structs",
            "Custom - Enums",
        ];
    }
    limpar_tela();

    match selecionado {
        1 => basicos::exemplo(),
        2 => custom::exemplo(),
        3 => sequencias::exemplo_struct(),
        4 => sequencias::exemplo_enum(),
        _ => break,
    }

    esperar_enter();
}
