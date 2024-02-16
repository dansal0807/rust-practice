use std::io;

fn main() {
    println!("Quantos números você quer comparar?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a linha.");

    let mut n: usize = input.trim().parse().unwrap_or_else(|_| {
        println!(".");
        std::process::exit(1);
    });

    println!("Me dê {} numeros:", n);
    let mut numbers = Vec::new(); // Vector to store the numbers

    while n != 0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a linha.");

        match input.trim().parse::<i32>() {
            Ok(num) => {
                numbers.push(num); // apenda o numero no vector
                n = n - 1; // Decrement n only if a valid number was entered
            },
            Err(_) => println!("Por favor, entre um número válido."),
        }
    }

    //vamos assumir que o maior numero é o primeiro da pilha.

    let mut max_number = numbers[0];

    for &num in &numbers[1..] {
        if num > max_number {
            max_number = num;
        }
    }

    println!("o maior numer é {}", max_number)

}
