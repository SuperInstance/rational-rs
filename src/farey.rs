//! Farey sequence generation.
//!
//! The Farey sequence F_n is the sorted sequence of all reduced fractions
//! between 0 and 1 with denominator ≤ n.

use crate::rational::Rational;

/// Generates the Farey sequence of order `n`.
///
/// Returns all reduced fractions `a/b` with `0 ≤ a ≤ b ≤ n`, sorted in ascending order.
/// Uses the recursive construction based on mediants.
pub fn farey_sequence(n: i64) -> Vec<Rational> {
    if n < 1 {
        return vec![Rational::new(0, 1)];
    }

    // Start with 0/1 and 1/1
    let mut seq = vec![Rational::new(0, 1), Rational::new(1, 1)];

    let mut i = 0;
    while i + 1 < seq.len() {
        let a = *seq[i].numer();
        let b = *seq[i].denom();
        let c = *seq[i + 1].numer();
        let d = *seq[i + 1].denom();

        // The mediant (a+c)/(b+d) belongs to F_n iff b+d ≤ n
        if b + d <= n {
            let mediant = Rational::new(a + c, b + d);
            seq.insert(i + 1, mediant);
            // Don't advance i; check if we can insert more between the same pair
        } else {
            i += 1;
        }
    }

    seq
}

/// Returns the number of terms in the Farey sequence F_n.
///
/// |F_n| = 1 + Σ_{k=1}^{n} φ(k) where φ is Euler's totient function.
pub fn farey_count(n: i64) -> usize {
    farey_sequence(n).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_farey_5() {
        let seq = farey_sequence(5);
        // F_5 = 0/1, 1/5, 1/4, 1/3, 2/5, 1/2, 3/5, 2/3, 3/4, 4/5, 1/1
        assert_eq!(seq.len(), 11);
        assert_eq!(seq[0], Rational::new(0, 1));
        assert_eq!(seq[seq.len() - 1], Rational::new(1, 1));
    }

    #[test]
    fn test_farey_all_unique() {
        let seq = farey_sequence(8);
        let mut seen = std::collections::HashSet::new();
        for q in &seq {
            assert!(seen.insert(q.clone()), "duplicate: {q}");
        }
    }
}
