use std::io;


fn convert_to_int(data_input: String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}

fn tabuada(number: i32) -> bool {
    for i in 1..=10 {
        let result = number * i;
        println!("{} x {} = {}" , number, i, result);
    }
    return true;
}

fn main() {
    println!("Me dê um número e te direi a tabuada:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Não foi possível ler o número");
    let numero = convert_to_int(num);

    tabuada(numero);

}
