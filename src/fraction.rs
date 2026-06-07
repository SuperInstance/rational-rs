//! Fraction operations: comparison, ordering, and conversion utilities.

use crate::rational::Rational;

/// Returns the absolute value of a rational number.
pub fn abs(q: Rational) -> Rational {
    if *q.numer() < 0 {
        -q
    } else {
        q
    }
}

/// Returns `true` if the rational is an integer.
pub fn is_integer(q: &Rational) -> bool {
    *q.denom() == 1
}

/// Converts the rational to an integer, if it is one.
pub fn to_integer(q: &Rational) -> Option<i64> {
    if is_integer(q) {
        Some(*q.numer())
    } else {
        None
    }
}

/// Returns the floor of a rational number.
pub fn floor(q: &Rational) -> i64 {
    let n = *q.numer();
    let d = *q.denom();
    if n >= 0 {
        n / d
    } else {
        // Floor division for negative numbers
        (n - d + 1) / d
    }
}

/// Returns the ceiling of a rational number.
pub fn ceil(q: &Rational) -> i64 {
    let n = q.numer();
    let d = q.denom();
    if *n >= 0 {
        (*n + d - 1) / d
    } else {
        *n / d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs_positive() {
        let q = Rational::new(3, 4);
        assert_eq!(abs(q), Rational::new(3, 4));
    }

    #[test]
    fn test_abs_negative() {
        let q = Rational::new(-3, 4);
        assert_eq!(abs(q), Rational::new(3, 4));
    }

    #[test]
    fn test_is_integer() {
        assert!(is_integer(&Rational::new(6, 3)));
        assert!(!is_integer(&Rational::new(5, 3)));
    }

    #[test]
    fn test_floor() {
        assert_eq!(floor(&Rational::new(7, 3)), 2);
        assert_eq!(floor(&Rational::new(-7, 3)), -3);
    }

    #[test]
    fn test_ceil() {
        assert_eq!(ceil(&Rational::new(7, 3)), 3);
        assert_eq!(ceil(&Rational::new(-7, 3)), -2);
    }
}
