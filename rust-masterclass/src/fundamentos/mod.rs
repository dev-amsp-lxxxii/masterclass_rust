use crate::utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

pub fn executar() {
    loop {
        let itens = [
            "Primeiro Exemplo",
            "Variáveis - Imutáveis",
            "Variáveis - Mutáveis",
            "Variáveis - Constantes",
            "Variáveis - Shadowing",
            "Operadores - Aritméticos",
            "Operadores - Relacionais",
            "Operadores - Lógicos",
        ];
        let selecionado = exibir_menu("Fundamentos", &itens, false);

        limpar_tela();

        match selecionado {
            1 => println!("Fundamentos - 1"),
            _ => break,
        }

        esperar_enter();
    }
}
