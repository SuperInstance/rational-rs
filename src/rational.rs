//! Core [`Rational`] type with reduced form arithmetic.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

/// A rational number in reduced form.
///
/// Internally stored as `numerator / denominator` where:
/// - `denominator > 0`
/// - `gcd(|numerator|, denominator) == 1`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rational {
    numer: i64,
    denom: i64,
}

impl Rational {
    /// Creates a new rational number, automatically reducing to lowest terms.
    ///
    /// # Panics
    /// Panics if `denom == 0`.
    pub fn new(numer: i64, denom: i64) -> Self {
        assert!(denom != 0, "denominator cannot be zero");
        let g = gcd(numer.unsigned_abs(), denom.unsigned_abs()) as i64;
        let g = if g == 0 { 1 } else { g };
        let numer = numer / g;
        let denom = denom / g;
        if denom < 0 {
            Rational { numer: -numer, denom: -denom }
        } else {
            Rational { numer, denom }
        }
    }

    /// Returns the numerator.
    pub fn numer(&self) -> &i64 { &self.numer }

    /// Returns the denominator (always positive).
    pub fn denom(&self) -> &i64 { &self.denom }

    /// Converts to an `f64`.
    pub fn to_f64(&self) -> f64 {
        self.numer as f64 / self.denom as f64
    }

    /// Creates a rational from an `f64` using continued fraction approximation.
    pub fn from_f64(x: f64) -> Self {
        if x == 0.0 {
            return Rational::new(0, 1);
        }
        let negative = x < 0.0;
        let x = x.abs();

        let cf = crate::continued::continued_fraction(x, 20);
        let convs = crate::continued::convergents(&cf);
        // Pick the last convergent that fits in i64
        let best = convs.last().copied().unwrap_or(Rational::new(0, 1));
        if negative {
            -best
        } else {
            best
        }
    }

    /// Returns the reciprocal `1/self`.
    pub fn reciprocal(&self) -> Self {
        assert!(self.numer != 0, "cannot take reciprocal of zero");
        Rational::new(self.denom, self.numer)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        // a/b vs c/d  →  a*d vs c*b
        let left = self.numer as i128 * other.denom as i128;
        let right = other.numer as i128 * self.denom as i128;
        left.cmp(&right)
    }
}

impl Add for Rational {
    type Output = Rational;
    fn add(self, other: Rational) -> Rational {
        Rational::new(
            self.numer * other.denom + other.numer * self.denom,
            self.denom * other.denom,
        )
    }
}

impl Sub for Rational {
    type Output = Rational;
    fn sub(self, other: Rational) -> Rational {
        Rational::new(
            self.numer * other.denom - other.numer * self.denom,
            self.denom * other.denom,
        )
    }
}

impl Mul for Rational {
    type Output = Rational;
    fn mul(self, other: Rational) -> Rational {
        Rational::new(self.numer * other.numer, self.denom * other.denom)
    }
}

impl Div for Rational {
    type Output = Rational;
    fn div(self, other: Rational) -> Rational {
        Rational::new(self.numer * other.denom, self.denom * other.numer)
    }
}

impl Neg for Rational {
    type Output = Rational;
    fn neg(self) -> Rational {
        Rational { numer: -self.numer, denom: self.denom }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.numer)
        } else {
            write!(f, "{}/{}", self.numer, self.denom)
        }
    }
}
