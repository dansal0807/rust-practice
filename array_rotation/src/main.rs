
fn rotacionar_array(array: &mut [i32], k: usize ) {

    let tamanho: usize = array.len();

    if tamanho == 0{
        return;
    }

    let k = k % tamanho; 
    array.reverse();
    array[0..k].reverse();
    array[k..].reverse();
}

fn main() {
    let mut array = [1, 2, 3, 4, 5, 6, 7];
    let k = 4;

    println!("Array original: {:?}", array);

    rotacionar_array(&mut array, k);

    println!("Array rotacionado: {:?}", array);
}