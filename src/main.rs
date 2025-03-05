use std::io; //  Biblioteca de entrada e saída

fn main() {
    println!("Guess the number!");

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
}
