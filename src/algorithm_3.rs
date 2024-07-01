use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    coeffs: Vec<f64>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<f64>) -> Self {
        Polynomial { coeffs }
    }

    #[allow(unused)] 
    pub fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    #[allow(unused)]
    pub fn leading_coefficient(&self) -> f64 {
        *self.coeffs.last().unwrap()
    }

    pub fn remove_trailing_zeros(&mut self) {
        while self.coeffs.len() > 1 && self.coeffs.last() == Some(&0.0) {
            self.coeffs.pop();
        }
    }

    //Horner's Method for evaluating polynomials
    pub fn evaluate(&self, x: f64) -> f64 {
        self.coeffs.iter().rev().fold(0.0, |acc, &coeff| acc * x + coeff)
    }

    pub fn multiply_by_constant(self, constant: f64) -> Self {
        let coeffs = self.coeffs.into_iter().map(|c| c * constant).collect();
        Polynomial::new(coeffs)
    }
}


impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut coeffs = vec![0.0; self.coeffs.len().max(other.coeffs.len())];
        for i in 0..self.coeffs.len() {
            coeffs[i] += self.coeffs[i];
        }
        for i in 0..other.coeffs.len() {
            coeffs[i] += other.coeffs[i];
        }
        let mut result = Polynomial::new(coeffs);
        result.remove_trailing_zeros();
        result
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut coeffs = vec![0.0; self.coeffs.len().max(other.coeffs.len())];
        for i in 0..self.coeffs.len() {
            coeffs[i] += self.coeffs[i];
        }
        for i in 0..other.coeffs.len() {
            coeffs[i] -= other.coeffs[i];
        }
        let mut result = Polynomial::new(coeffs);
        result.remove_trailing_zeros();
        result
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut coeffs = vec![0.0; self.coeffs.len() + other.coeffs.len() - 1];
        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                coeffs[i + j] += self.coeffs[i] * other.coeffs[j];
            }
        }
        let mut result = Polynomial::new(coeffs);
        result.remove_trailing_zeros();
        result
    }
}

//algorithm 3 - Polynomial Euclideam Algorithm
#[allow(non_snake_case)]
#[allow(unused)] 
pub fn pea(A: Polynomial, B: Polynomial) -> (Polynomial, Polynomial) {
    let mut Q = Polynomial::new(vec![0.0]);
    let mut P = A.clone();
    let d = B.degree();
    let c = B.leading_coefficient();

    while P.degree() >= d {
        let mut S_coeffs = vec![0.0; P.degree() - d + 1];
        S_coeffs[P.degree() - d] = P.leading_coefficient() / c;
        let S = Polynomial::new(S_coeffs);
        Q = Q + S.clone();
        P = P - (S * B.clone());
    }

    (Q, P)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_pea() {
        let A = Polynomial::new(vec![-9.0, 0.0, 0.0, 2.0, 0.0, 1.0]);
        let B = Polynomial::new(vec![-1.0 , 4.0, 1.0]);
        let (Q, P) = pea(A, B);
        assert_eq!(Q, Polynomial::new(vec![-80.0 , 19.0, -4.0, 1.0]));
        assert_eq!(P, Polynomial::new(vec![-89.0, 339.0]));
    }
}