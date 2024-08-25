use crate::field::FieldElement;

use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops::{Add, Mul};

// Represents a univariate polynomial over a prime field.
#[derive(Clone, Debug)]
pub struct Polynomial {
    coefficients: Vec<FieldElement>,
    prime: BigInt,
}

impl Polynomial {
    // Creates a new polynomial from the given coefficients.
    pub fn new(coefficients: Vec<FieldElement>, prime: BigInt) -> Self {
        // Ensure all coefficients are in the same field
        for coef in &coefficients {
            assert_eq!(coef.prime(), &prime);
        }
        Self {
            coefficients,
            prime,
        }
    }

    // Evaluates the polynomial at a given field element, x.
    pub fn evaluate(&self, x: &FieldElement) -> FieldElement {
        let mut y = FieldElement::new(BigInt::zero(), self.prime.clone());
        let mut exponent = FieldElement::new(BigInt::one(), self.prime.clone());

        for coeff in &self.coefficients {
            y += coeff.clone() * exponent.clone();
            exponent = exponent * x.clone();
        }

        y
    }
}

impl Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Self::Output {
        assert_eq!(self.prime, other.prime);

        // Initialize the coefficients to the maximum length of the two polynomials.
        let mut coefficients = vec![
            FieldElement::new(BigInt::zero(), self.prime.clone());
            usize::max(self.coefficients.len(), other.coefficients.len())
        ];

        // Add the coefficients of the two polynomials.
        for (i, coeff) in self.coefficients.iter().enumerate() {
            coefficients[i] += coeff.clone();
        }
        for (i, coeff) in other.coefficients.iter().enumerate() {
            coefficients[i] += coeff.clone();
        }

        Polynomial::new(coefficients, self.prime.clone())
    }
}

impl Mul<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Self::Output {
        assert_eq!(self.prime, other.prime);

        // Initialize the coefficients to the sum of the lengths of the two polynomials.
        let mut new_coefficients = vec![
            FieldElement::new(BigInt::zero(), self.prime.clone());
            self.coefficients.len() + other.coefficients.len() - 1
        ];

        // Multiply the coefficients of the two polynomials.
        for (i, coeff_self) in self.coefficients.iter().enumerate() {
            for (j, coeff_other) in other.coefficients.iter().enumerate() {
                new_coefficients[i + j] += coeff_self.clone() * coeff_other.clone();
            }
        }

        Polynomial::new(new_coefficients, self.prime.clone())
    }
}

impl Mul<FieldElement> for Polynomial {
    type Output = Polynomial;

    fn mul(self, element: FieldElement) -> Self::Output {
        let new_coefficients = self
            .coefficients
            .iter()
            .map(|coeff| coeff.clone() * element.clone())
            .collect();
        Polynomial::new(new_coefficients, self.prime.clone())
    }
}
