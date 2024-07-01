//algorithm 1 - Extended Euclidean Algorithm
pub fn EEA(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut u, mut g, mut x, mut y) = (1, a, 0, b);

    while y != 0 {
        let q = g / y;
        let t = g % y;

        let s = u - q * x;
        u = x;
        g = y;
        x = s;
        y = t;
    }

    let v = (g - a * u) / b;
    (g, u, v)
}

//algorithm 2 - Chinese Remainder Theorem
pub fn CRT(a: &[i64], n: &[i64]) -> i64 { 
    let k = a.len();
    let mut N = 1;
    for &n_i in n {
        N *= n_i;
    }

    let mut x = 0;
    for j in 0..k {
        let N_j = N / n[j];
        let (_, s_j, t_j) = EEA(N_j, n[j]);
        x += a[j] * s_j * N_j;
    }

    x % N
}