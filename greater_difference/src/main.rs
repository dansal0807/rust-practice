use std::io;

fn main() {
    let mut input = String::new();
    let mut numbers = Vec::new();

    println!("Diga os valores das suas ações (digite 'done' para finalizar):");

    loop {
        input.clear(); // Clear the input string to receive new input
        io::stdin().read_line(&mut input).expect("Falha ao ler seu input.");

        if input.trim().eq_ignore_ascii_case("done") {
            break; // Exit the loop if the user types "done"
        }

        match input.trim().parse::<i32>() {
            Ok(num) => numbers.push(num), // Add the number to the vector
            Err(_) => println!("Insira um número válido ou 'done' para finalizar."),
        }
    }

    if numbers.is_empty() {
        println!("Nenhum número foi inserido.");
        return;
    }

    let result = find_max_difference(&numbers);
    println!("O maior lucro possível é: {}", result);
}

fn find_max_difference(numbers: &Vec<i32>) -> i32 {
    if numbers.is_empty() {
        return 0;
    }

    let (min_index, &min_value) = numbers.iter().enumerate().min_by_key(|&(_, &val)| val).unwrap_or((0, &numbers[0]));

    if min_index == numbers.len() - 1 {
        return 0;
    }

    let max_value = numbers.iter().skip(min_index + 1).max().unwrap_or(&min_value);

    max_value - min_value
}
