use std::io;

fn convert_to_int(data_input: &str) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    return x
}

fn main() {

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler a entrada.");
    let fatorial = convert_to_int(&entrada) + 1;
    let mut numero = 1_i64; //se nao colocarmos o _i64, só conseguimos fatorial até 7.

    for i in 1..fatorial{
        numero *= i as i64;
    }

    println!("o fatorial de {} é {}", entrada.trim(), numero);

}
