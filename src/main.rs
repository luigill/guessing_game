use rand::Rng;
use std::io; //  Biblioteca de entrada e saída // Biblioteca de números randomizados do crates.io

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess...");

    // Variáveis por padrão são imutáveis -> Não mudam de valor durante execução
    // let apples = 5 -> Sempre será 5

    // Usando-se a keyword mut,  faz com que seja possível alterar o valor de uma variável
    let mut guess = String::new(); // Define uma variável string -> Nova instância de um objeto

    io::stdin() // chamada de IO
        .read_line(&mut guess) // Lê a string e aloca por referência no endereço da variável
        .expect("Failed to read line"); // Trata o erro caso não seja possível ler
    // Sem o expect, o compilador retornaria um erro alertando sobre uma possível exceção

    println!("You guessed: {}", guess);

    // TODO: Comparação entre números
}
