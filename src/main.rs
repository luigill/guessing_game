use rand::Rng; // Biblioteca de números randomizados do crates.io
use std::cmp::Ordering; // Comparação entre valores
use std::io; //  Biblioteca de entrada e saída

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop infinito
    loop {
        println!("Please input your guess...");

        // Variáveis por padrão são imutáveis -> Não mudam de valor durante execução
        // let apples = 5 -> Sempre será 5

        // Usando-se a keyword mut,  faz com que seja possível alterar o valor de uma variável
        let mut guess = String::new(); // Define uma variável string -> Nova instância de um objeto

        io::stdin() // chamada de IO
            .read_line(&mut guess) // Lê a string e aloca por referência no endereço da variável
            .expect("Failed to read line"); // Trata o erro caso não seja possível ler
        // Sem o expect, o compilador retornaria um erro alertando sobre uma possível exceção

        // Se reusa o nome guess -> Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //trim = retira espaços em branco
        //parse = Converte a variável para o novo tipo definido

        println!("You guessed: {}", guess);

        // match -> Faz alguma ação baseada em um padrão
        // uma ou mais ações
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // Projeto Finalizado!
}
