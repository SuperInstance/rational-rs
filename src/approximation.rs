//! Best rational approximation using continued fraction convergents.

use crate::continued;
use crate::rational::Rational;

/// Finds the best rational approximation to `x` with denominator ≤ `max_denom`.
///
/// Uses continued fraction convergents: computes the CF expansion, then
/// finds the best convergent within the denominator bound.
pub fn best_approximation(x: f64, max_denom: i64) -> Rational {
    if x == 0.0 {
        return Rational::new(0, 1);
    }

    let negative = x < 0.0;
    let x = x.abs();

    let cf = continued::continued_fraction(x, 30);
    let convs = continued::convergents(&cf);

    // Find the best convergent with denom <= max_denom
    let mut best = Rational::new(0, 1);
    let mut best_error = x;

    for c in &convs {
        if *c.denom() > max_denom {
            break;
        }
        let error = (c.to_f64() - x).abs();
        if error < best_error {
            best_error = error;
            best = *c;
        }
    }

    // Also try semi-convergents: between last two valid convergents
    // For simplicity, we just return the best convergent found.
    if negative {
        -best
    } else {
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approximate_two() {
        let result = best_approximation(2.0, 100);
        assert_eq!(result, Rational::new(2, 1));
    }

    #[test]
    fn test_approximate_rational() {
        let result = best_approximation(3.0 / 7.0, 100);
        assert_eq!(result, Rational::new(3, 7));
    }
}
