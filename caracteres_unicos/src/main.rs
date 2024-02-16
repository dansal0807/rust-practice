fn has_repeated_letters(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<char>>(); // Criando um vetor de chars.
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            if chars[i] == chars[j] {
                return true; // Se encontrar um char igual, retorna verdadeiro.
            }
        }
    }
    return false // Nenhum char encontrado, retorna falso.
}

fn main() {
    let string_teste = "hello";
    if has_repeated_letters(string_teste) {
        println!("'{}' tem elementos repetidos.", string_teste);
    } else {
        println!("'{}' não tem elementos repetidos.", string_teste);
    }
}
