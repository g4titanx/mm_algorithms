use crate::algorithm_3::Polynomial;

//algorithm 4 - lagrange interpolation
#[allow(non_snake_case)]
#[allow(unused)]
pub fn lagrange_interpolation(points: &[(f64, f64)]) -> Polynomial {
    let m = points.len() - 1;
    let mut P = Polynomial::new(vec![0.0]);

    for j in 0..=m {
        let (x_j, y_j) = points[j];
        let mut l_j = Polynomial::new(vec![1.0]);

        for (i, &(x_i, _)) in points.iter().enumerate() {
            if i != j {
                let denominator = x_j - x_i;
                let term = Polynomial::new(vec![-x_i, 1.0]).multiply_by_constant(1.0 / denominator);
                l_j = l_j * term;
            }
        }

        P = P + l_j.multiply_by_constant(y_j);
    }

    P
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_lagrange_interpolation() {
        let points = vec![(0.0, 4.0), (-2.0, 1.0), (2.0, 3.0)];
        let P = lagrange_interpolation(&points);

        let expected_coeffs = vec![4.0, 0.5, -0.5];
        assert_eq!(expected_coeffs, P.coeffs);
    }
}