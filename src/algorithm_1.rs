//algorithm 1 - Extended Euclidean Algorithm
pub fn eea(a: i64, b: i64) -> (i64, i64, i64) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eea () {
        let a = 76;
        let b = 43;
        let result = eea(a, b);
        assert_eq!(result, (1, -13, 23));
    }
}