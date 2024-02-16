use std::collections::HashMap;

fn eh_permutacao(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false; // Strings of different lengths cannot be permutations
    }

    let mut counts = HashMap::new();

    // Count characters in str1
    for ch in str1.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    // Subtract counts for characters in str2
    for ch in str2.chars() {
        let count = counts.entry(ch).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false; // More occurrences of ch in str2 than in str1
        }
    }

    true // All character counts match
}

fn main() {
    let str1 = "hello";
    let str2 = "hallo";
    
    if eh_permutacao(str1, str2) {
        println!("As strings são permutações uma da outra.");
    } else {
        println!("As strings não são permutações uma da outra.");
    }
}
