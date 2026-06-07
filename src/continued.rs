//! Continued fraction expansion and convergents.

use crate::rational::Rational;

/// Computes the simple continued fraction expansion of a positive `f64` value.
///
/// Returns a vector of integer terms `[a₀; a₁, a₂, ...]`.
/// The expansion is truncated after `max_terms` or when the remainder is negligible.
pub fn continued_fraction(x: f64, max_terms: usize) -> Vec<i64> {
    if x == 0.0 {
        return vec![0];
    }

    let mut terms = Vec::with_capacity(max_terms);
    let mut remainder = x;

    for _ in 0..max_terms {
        let a = remainder.floor() as i64;
        terms.push(a);
        let frac = remainder - a as f64;
        if frac < 1e-14 {
            break;
        }
        remainder = 1.0 / frac;
    }

    terms
}

/// Computes the convergents (rational approximations) from a continued fraction expansion.
///
/// Returns a vector of rationals p_k/q_k where each is a progressively better
/// approximation to the original value.
pub fn convergents(cf: &[i64]) -> Vec<Rational> {
    if cf.is_empty() {
        return vec![];
    }

    let mut convs = Vec::with_capacity(cf.len());

    // p_{-2} = 0, p_{-1} = 1
    // q_{-2} = 1, q_{-1} = 0
    let mut p_prev2: i64 = 0;
    let mut p_prev1: i64 = 1;
    let mut q_prev2: i64 = 1;
    let mut q_prev1: i64 = 0;

    for &a in cf {
        let p = a * p_prev1 + p_prev2;
        let q = a * q_prev1 + q_prev2;
        if q != 0 {
            convs.push(Rational::new(p, q));
        }
        p_prev2 = p_prev1;
        p_prev1 = p;
        q_prev2 = q_prev1;
        q_prev1 = q;
    }

    convs
}

/// Evaluates a continued fraction to a rational number.
pub fn evaluate(cf: &[i64]) -> Rational {
    let convs = convergents(cf);
    convs.last().copied().unwrap_or(Rational::new(0, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cf_integer() {
        let cf = continued_fraction(3.0, 10);
        assert_eq!(cf, vec![3]);
    }

    #[test]
    fn test_cf_one_half() {
        let cf = continued_fraction(0.5, 10);
        assert_eq!(cf, vec![0, 2]);
    }

    #[test]
    fn test_convergents_simple() {
        let cf = vec![3, 7]; // 3 + 1/7 = 22/7
        let convs = convergents(&cf);
        assert_eq!(convs[0], Rational::new(3, 1));
        assert_eq!(convs[1], Rational::new(22, 7));
    }

    #[test]
    fn test_evaluate_cf() {
        let cf = vec![0, 2]; // 0 + 1/2 = 1/2
        assert_eq!(evaluate(&cf), Rational::new(1, 2));
    }
}
