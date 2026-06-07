//! # rational-rs
//!
//! Rational number library with reduced form arithmetic, continued fractions,
//! best rational approximation, and Farey sequences.
//!
//! ## Modules
//! - [`rational`] - Core `Rational` type and reduced form arithmetic
//! - [`fraction`] - Fraction operations: comparison, ordering, conversion
//! - [`continued`] - Continued fraction expansion and convergents
//! - [`approximation`] - Best rational approximation (Stern-Brocot / binary search)
//! - [`farey`] - Farey sequence generation

pub mod approximation;
pub mod continued;
pub mod farey;
pub mod fraction;
pub mod rational;

pub use rational::Rational;

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-10;

    fn r(n: i64, d: i64) -> Rational {
        Rational::new(n, d)
    }

    // ── Arithmetic tests ──────────────────────────────────────

    #[test]
    fn test_create() {
        let q = r(3, 4);
        assert_eq!(*q.numer(), 3);
        assert_eq!(*q.denom(), 4);
    }

    #[test]
    fn test_create_reduces() {
        let q = r(6, 8);
        assert_eq!(*q.numer(), 3);
        assert_eq!(*q.denom(), 4);
    }

    #[test]
    fn test_create_negative_denom() {
        let q = r(3, -4);
        assert_eq!(*q.numer(), -3);
        assert_eq!(*q.denom(), 4);
    }

    #[test]
    fn test_zero() {
        let q = r(0, 5);
        assert_eq!(*q.numer(), 0);
        assert_eq!(*q.denom(), 1);
    }

    #[test]
    fn test_one() {
        let q = r(5, 5);
        assert_eq!(*q.numer(), 1);
        assert_eq!(*q.denom(), 1);
    }

    #[test]
    fn test_add() {
        let a = r(1, 3);
        let b = r(1, 6);
        let sum = a + b;
        assert_eq!(sum, r(1, 2));
    }

    #[test]
    fn test_add_reduces() {
        let a = r(1, 6);
        let b = r(1, 6);
        assert_eq!(a + b, r(1, 3));
    }

    #[test]
    fn test_sub() {
        let a = r(1, 2);
        let b = r(1, 3);
        assert_eq!(a - b, r(1, 6));
    }

    #[test]
    fn test_mul() {
        let a = r(2, 3);
        let b = r(3, 4);
        assert_eq!(a * b, r(1, 2));
    }

    #[test]
    fn test_div() {
        let a = r(2, 3);
        let b = r(4, 5);
        assert_eq!(a / b, r(5, 6));
    }

    #[test]
    fn test_neg() {
        let a = r(3, 4);
        assert_eq!(-a, r(-3, 4));
    }

    #[test]
    fn test_reciprocal() {
        let a = r(3, 4);
        assert_eq!(a.reciprocal(), r(4, 3));
    }

    #[test]
    fn test_reciprocal_negative() {
        let a = r(-3, 4);
        assert_eq!(a.reciprocal(), r(-4, 3));
    }

    #[test]
    fn test_to_f64() {
        let q = r(1, 3);
        assert!((q.to_f64() - 1.0 / 3.0).abs() < TOL);
    }

    #[test]
    fn test_from_f64() {
        let q = Rational::from_f64(0.5);
        assert_eq!(q, r(1, 2));
    }

    #[test]
    fn test_from_f64_repeating() {
        let q = Rational::from_f64(1.0 / 3.0);
        // Should approximate 1/3
        assert!((q.to_f64() - 1.0 / 3.0).abs() < 1e-10);
    }

    // ── Continued fraction tests ──────────────────────────────

    #[test]
    fn test_cf_sqrt2() {
        // √2 = [1; 2, 2, 2, ...]
        let cf = continued::continued_fraction(2.0_f64.sqrt(), 10);
        assert_eq!(cf[0], 1);
        for &term in &cf[1..] {
            assert_eq!(term, 2);
        }
    }

    #[test]
    fn test_cf_golden_ratio() {
        // φ = [1; 1, 1, 1, ...]
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let cf = continued::continued_fraction(phi, 10);
        for &term in &cf {
            assert_eq!(term, 1);
        }
    }

    #[test]
    fn test_cf_rational() {
        // 355/113 = [3; 7, 16]
        let cf = continued::continued_fraction(355.0 / 113.0, 20);
        assert_eq!(cf[0], 3);
        assert_eq!(cf[1], 7);
        // The exact expansion depends on floating point
        assert!(cf.len() >= 3);
    }

    #[test]
    fn test_convergents_pi() {
        let cf = continued::continued_fraction(std::f64::consts::PI, 10);
        let convs = continued::convergents(&cf);
        // Known convergents of π: 3/1, 22/7, 333/106, 355/113
        assert_eq!(convs[0], r(3, 1));
        assert!(convs.len() >= 2);
        // 22/7
        let found_22_7 = convs.iter().any(|c| *c == r(22, 7));
        assert!(found_22_7);
    }

    #[test]
    fn test_cf_roundtrip() {
        let value = 22.0 / 7.0;
        let cf = continued::continued_fraction(value, 20);
        let convs = continued::convergents(&cf);
        let last = convs.last().unwrap();
        assert!((last.to_f64() - value).abs() < 1e-10);
    }

    // ── Approximation tests ───────────────────────────────────

    #[test]
    fn test_approximate_pi() {
        let result = approximation::best_approximation(std::f64::consts::PI, 100);
        assert_eq!(result, r(22, 7)); // Classic approximation
    }

    #[test]
    fn test_approximate_pi_precise() {
        let result = approximation::best_approximation(std::f64::consts::PI, 1000);
        assert_eq!(result, r(355, 113)); // More precise approximation
    }

    #[test]
    fn test_approximate_e() {
        let e = std::f64::consts::E;
        let result = approximation::best_approximation(e, 100);
        // 19/7 ≈ 2.714... and 87/32 ≈ 2.71875
        assert!((result.to_f64() - e).abs() < 0.01);
    }

    #[test]
    fn test_approximate_one_half() {
        let result = approximation::best_approximation(0.5, 100);
        assert_eq!(result, r(1, 2));
    }

    #[test]
    fn test_approximate_zero() {
        let result = approximation::best_approximation(0.0, 100);
        assert_eq!(result, r(0, 1));
    }

    // ── Farey sequence tests ──────────────────────────────────

    #[test]
    fn test_farey_1() {
        let seq = farey::farey_sequence(1);
        assert_eq!(seq, vec![r(0, 1), r(1, 1)]);
    }

    #[test]
    fn test_farey_2() {
        let seq = farey::farey_sequence(2);
        assert_eq!(seq, vec![r(0, 1), r(1, 2), r(1, 1)]);
    }

    #[test]
    fn test_farey_3() {
        let seq = farey::farey_sequence(3);
        assert_eq!(seq, vec![r(0, 1), r(1, 3), r(1, 2), r(2, 3), r(1, 1)]);
    }

    #[test]
    fn test_farey_4() {
        let seq = farey::farey_sequence(4);
        assert_eq!(seq, vec![r(0, 1), r(1, 4), r(1, 3), r(1, 2), r(2, 3), r(3, 4), r(1, 1)]);
    }

    #[test]
    fn test_farey_sorted() {
        let seq = farey::farey_sequence(7);
        for window in seq.windows(2) {
            assert!(window[0] < window[1]);
        }
    }

    #[test]
    fn test_farey_count() {
        // |F_n| = 1 + sum_{k=1}^{n} φ(k) where φ is Euler's totient
        let seq = farey::farey_sequence(5);
        // |F_5| = 1 + φ(1) + φ(2) + φ(3) + φ(4) + φ(5) = 1 + 1 + 1 + 2 + 2 + 4 = 11
        assert_eq!(seq.len(), 11);
    }

    #[test]
    fn test_farey_neighbors_property() {
        // For consecutive terms a/b and c/d in F_n: bc - ad = 1
        let seq = farey::farey_sequence(6);
        for window in seq.windows(2) {
            let a = *window[0].numer();
            let b = *window[0].denom();
            let c = *window[1].numer();
            let d = *window[1].denom();
            assert_eq!(b * c - a * d, 1);
        }
    }

    // ── Comparison tests ──────────────────────────────────────

    #[test]
    fn test_comparison() {
        assert!(r(1, 3) < r(1, 2));
        assert!(r(1, 2) < r(2, 3));
        assert!(r(3, 4) > r(1, 2));
        assert!(r(1, 2) == r(2, 4));
    }

    #[test]
    fn test_comparison_negative() {
        assert!(r(-1, 2) < r(1, 3));
        assert!(r(-2, 3) < r(-1, 2));
    }
}
