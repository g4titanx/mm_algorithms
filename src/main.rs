use mm_algorithms::*;

fn main() {
    let a = vec![4, 1, 3, 0]; // example values for a
    let n = vec![7, 3, 5, 11]; //example values for n

    let result = crt(&a, &n);
    println!("{}", result);
}