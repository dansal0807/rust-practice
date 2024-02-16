use std::io;

fn main() {
    println!("Quantos números você quer somar?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a linha.");

    let mut n: usize = input.trim().parse().unwrap_or_else(|_| {
        println!(".");
        std::process::exit(1);
    });

    println!("Me dê {} numeros:", n);
    let mut numbers = Vec::new(); // Vetor para somar os números.

    while n != 0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a linha.");

        match input.trim().parse::<i32>() {
            Ok(num) => {
                numbers.push(num); // apenda o numero no vector
                n = n - 1; // Descresce um número caso o numero nao tenha sido incluido.
            },
            Err(_) => println!("Por favor, entre um número válido."),
        }
    }

    //vamos percorrer o vetor e somar cada item.
    let mut result = 0;
    for num in &numbers {
        if *num % 2 == 0 {
            result += *num;
        }
        }
    println!("a soma é {}", result);
    }
