#![allow(dead_code)]

use std::ops::Div;
use std::ops::Mul;

#[derive(Debug, PartialEq, Eq)]
pub struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        if denominator == 0 {
            panic!("denominator cannot be 0");
        }

        let gcd = gcd(numerator, denominator);
        Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        // Dividing with a fraction is the same as multiplying with the
        // inverted fraction
        let numerator = self.numerator * rhs.denominator;
        let denominator = self.denominator * rhs.numerator;

        // Can never be None, since neither of the multiplied fractions can
        // be invalid
        Self::new(numerator, denominator)
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;

        // Can never be None, since neither of the multiplied fractions can
        // be invalid
        Self::new(numerator, denominator)
    }
}

// Find greatest common denominator
fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greatest_common_denominator() {
        assert_eq!(gcd(3, 9), 3);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(13, 17), 1);
    }

    #[test]
    #[should_panic]
    fn creating_invalid_fraction() {
        let _ = Fraction::new(0, 0);
    }

    fn creating_valid_fraction() {
        let _ = Fraction::new(0, 2);
        let _ = Fraction::new(27, 3);
    }

    #[test]
    fn multiplying_fractions() {
        assert_eq!(
            Fraction::new(2, 2) * Fraction::new(2, 2),
            Fraction::new(1, 1)
        );
        assert_eq!(
            Fraction::new(3, 1) * Fraction::new(1, 3),
            Fraction::new(1, 1)
        );
    }

    #[test]
    fn dividing_fractions() {
        assert_eq!(
            Fraction::new(2, 2) / Fraction::new(2, 2),
            Fraction::new(1, 1)
        );
        assert_eq!(
            Fraction::new(3, 1) / Fraction::new(1, 3),
            Fraction::new(9, 1)
        );
        assert_eq!(
            Fraction::new(3, 1) / Fraction::new(3, 1),
            Fraction::new(1, 1)
        );
        assert_eq!(
            Fraction::new(2, 1) / Fraction::new(1, 2),
            Fraction::new(4, 1)
        );
        assert_eq!(
            Fraction::new(13, 1) / Fraction::new(17, 1),
            Fraction::new(13, 17)
        );
    }
}
