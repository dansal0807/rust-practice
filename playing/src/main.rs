

fn main() {

    let numeros: [i32;5] = [1,2,3,100,5];
    println!("{}", numeros[3]);
    for i in numeros.iter() {
        println!("{}", i);
    }
}