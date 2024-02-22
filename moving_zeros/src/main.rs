/*Dado um vetor de inteiros, escreva uma função para mover todos os 0's para o final dele, 
mantendo a ordem relativa dos elementos não zero em Rust.

Por exemplo, dado nums = [0, 1, 0, 3, 12], após chamar sua função, nums deve ficar [1, 3, 12, 0, 0].

Nota: Você deve fazer isso in-place, sem criar uma cópia do array. Minimize o número total de operações.*/

fn main() {
    let mut  nums = vec![0, 1, 0, 3, 12];
    moving_zeros(&mut nums);
    println!("novo nums: {:?}", nums);

    
}

fn moving_zeros(numbers: &mut Vec<i32>) {

    let mut position = 0;

    for i in 0..numbers.len() {
        if numbers[i] != 0 {
            numbers[position] = numbers[i];
            if i != position {
                numbers[i] = 0;
            }
            position += 1;


        } 
        }
    }
    

