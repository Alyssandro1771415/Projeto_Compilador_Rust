fn calcular_bonus(salario: f64, meses: i32) -> f64 {
    let mut resultado: f64 = salario * 1.15;
    resultado
}

use std::io;

mod processamento {
    fn status() {
        println!("Processando dados...");
    }
}

fn main() {
    let nome: str = "João Pedro";
    let idade: i32 = 25;
    let pi: f64 = 3.1415;
    let ativo: bool = true;

    println!("Digite um valor:");
    read_line!(input_usuario);

    let calculo: i32 = (10 + 5) * 2 / 3 % 2;

    if idade >= 18 && ativo == true {
        println!("Acesso permitido");
    } else if idade < 0 || !ativo {
        println!("Erro de cadastro");
    } else {
        println!("Acesso negado");
    }

    let mut contador: i32 = 0;
    while contador < 5 {
        contador = contador + 1;
        println!("Contagem...");
    }

    let bonus: f64 = calcular_bonus(2500.0, 12);
}