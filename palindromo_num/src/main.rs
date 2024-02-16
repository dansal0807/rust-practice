fn is_palindrome(number: i32) -> bool {
    let num_str = number.to_string(); 
    let rev_str = num_str.chars().rev().collect::<String>(); 
    num_str == rev_str 
}

fn main() {
    let num: i32 = -121;
    if is_palindrome(num) {
        println!("{} is a palindrome.", num);
    } else {
        println!("{} is not a palindrome.", num);
    }

    let num: i32 = 101;
    if is_palindrome(num) {
        println!("{} is a palindrome.", num);
    } else {
        println!("{} is not a palindrome.", num);
    }
}