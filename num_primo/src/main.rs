use std::io;


fn convert_to_int(data_input: &mut str) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}

fn eh_primo(number: &i32) -> bool {
    if *number <= 1 {
        println!("O número 1, 0 ou negativo é considerado como não-primo.");
        return false; // Numbers less than or equal to 1 are not prime.
    }

    let limite = number * number;
    for i in 2..limite {
        if *number % i == 0 {
            println!("O numero {} não é primo!", number);
            return false;
        } else {
            println!("O numero {} é primo!", number);
            return true;
        }
    }
    return true;
}

fn main() {
    println!("Me dê um número:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Não foi possível ler o número");
    let primo = convert_to_int(&mut num);

    eh_primo(&primo);

}
