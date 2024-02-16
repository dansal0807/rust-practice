use std::io;
use std::cmp::max;

fn convert_to_int(data_input: &str) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
//a ideia é que o mmc é o menor numero divisível por ambos os numeros. 
    let mut input = String::new();
    println!("Entre com o primeiro número:");
    io::stdin().read_line(&mut input).expect("Erro ao ler o número.");
    let a = convert_to_int(&input);
    input.clear();

    println!("Entre com o segundo número:");
    let _ = io::stdin().read_line(&mut input);
    let b = convert_to_int(&input);

    let mut mcm = max(a, b); // aqui encontramos o maior numero entre a e b.

    while mcm % a != 0 || mcm % b != 0 {
        mcm += 1; //aumentamos em 1 unidade até o numero ser divisivel por ambos
    }

    println!("O mmc entre {} e {} é {}", a, b, mcm)
}
