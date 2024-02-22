use std::collections::HashMap;

fn main() {
    let mut nomes = HashMap::new();

    nomes.insert("Daniel", 27);
    nomes.insert("Gabriela", 32);

    println!("As idades de Daniel e gabriela são {} e {}", nomes.get("Daniel").unwrap(), nomes.get("Gabriela").unwrap());

    for keys in nomes.keys() {
        println!("{}", keys);
    }

    for value in nomes.values() {
        println!("{}", value);
    }
}