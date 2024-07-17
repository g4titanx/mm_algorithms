//algorithm 5 - Cyclic Group Exponentiation/ Square and Multiply
//assuming G is a multiplicative group
pub fn exponentiation(g: u64, x: u64, n: u64) -> u64 {
    let mut h = g;
    let mut y = 1;
    let binary_x = format!("{:b}", x); 
    let k = binary_x.len();

    for j in 0..k {
        let bj = binary_x.chars().nth(k - j - 1).unwrap();

        if bj == '1' {
            y = (y * h) % n; 
        }
        h = (h * h) % n;
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponentiation() {
        let g = 2; 
        let x = 13;
        let n = 17;
    
        let result = exponentiation(g, x, n);
        assert_eq!(result, 15);
    }
}