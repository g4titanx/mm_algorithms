use crate::algorithm_1::eea;

#[allow(non_snake_case)]
#[allow(unused)]
//algorithm 2 - Chinese Remainder Theorem
pub fn crt(a: &[i64], n: &[i64]) -> i64 { 
    let k = a.len();
    let mut N = 1;
    for &n_i in n {
        N *= n_i;
    }

    let mut x = 0;
    for j in 0..k {
        let N_j = N / n[j];
        let (_, s_j, t_j) = eea(N_j, n[j]);
        x += a[j] * s_j * N_j;
    }

    x % N
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt () {
        let a = vec![4, 1, 3, 0];
        let n = vec![7, 3, 5, 11];
        let result = crt(&a, &n);
        assert_eq!(result, 88);
    }
}