use std::io;

fn calcula_media(notas: Vec<i32>) -> f32 {
    let sum: f32 = notas.iter().map(|&nota| nota as f32).sum(); // Convert i32 to f32 and sum
    let len = notas.len() as f32; // Convert length to f32 for floating point division
    sum / len // Return the average
}

fn main() {
    println!("Quantos numeros você deseja criar uma média?");

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

    let media = calcula_media(numbers);
    println!("A média das notas é {}", media);
    }
