use std::io;

fn convert_to_int(data_input: &str) -> Result<i32, std::num::ParseIntError> {
    data_input.trim().parse()
}

fn main() {
    println!("Please enter a number (0 to exit):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");

        let num = match convert_to_int(&input) {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid integer.");
                continue;
            },
        };

        if num == 0 {
            println!("Exiting.");
            break;
        }

        let sum = sum_digits(num);
        println!("The sum of the digits is: {}", sum);
    }
}

fn sum_digits(mut num: i32) -> i32 {
    let mut sum = 0;
    while num != 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}
