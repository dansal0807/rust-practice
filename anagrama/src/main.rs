fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();

    // Sort both vectors of characters
    s_chars.sort();
    t_chars.sort();

    // Compare if the sorted vectors are equal
    if s_chars == t_chars {
        return true
    } else {
        return false
    }
}

fn main() {
    let s1 = "anagram";
    let t1 = "nagaram";
    println!("{}", is_anagram(s1, t1)); // Output: true

    let s2 = "rat";
    let t2 = "car";
    println!("{}", is_anagram(s2, t2)); // Output: false
}
