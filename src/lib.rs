use std::num::NonZeroU32;

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
pub fn CRT(k: i32, j: u32, n: &[Option<NonZeroU32>; k-1] ) -> { 
    
}