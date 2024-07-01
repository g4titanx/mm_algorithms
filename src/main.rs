fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_euclid(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn chinese_remainder_theorem(a: &[i64], n: &[i64]) -> i64 {
    let k = a.len();
    let mut N = 1;
    for &ni in n {
        N *= ni;
    }

    let mut x = 0;
    for j in 0..k {
        let Nj = N / n[j];
        let (_, sj, _) = extended_euclid(Nj, n[j]);
        x += a[j] * sj * Nj;
    }

    x % N
}

fn main() {
    let a = vec![4, 1, 3, 0]; // example values for a
    let n = vec![7, 3, 5, 11]; // example values for n

    let result = chinese_remainder_theorem(&a, &n);
    println!("The solution is: {}", result);
}