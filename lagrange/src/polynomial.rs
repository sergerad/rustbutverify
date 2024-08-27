use crate::field::FieldElement;

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use num_bigint::BigInt;
use std::ops::{Add, Mul};

// Represents a univariate polynomial over a prime field.
#[derive(Clone, Debug)]
pub struct Polynomial<'a> {
    coefficients: Vec<FieldElement<'a>>,
    prime: &'a BigInt,
}

impl<'a> Polynomial<'a> {
    // Creates a new polynomial from the given coefficients.
    pub fn new(coefficients: Vec<FieldElement<'a>>, prime: &'a BigInt) -> Self {
        // Ensure all coefficients are in the same field
        for coef in &coefficients {
            assert_eq!(coef.prime(), prime);
        }
        Self {
            coefficients,
            prime,
        }
    }

    // Evaluates the polynomial at a given field element, x.
    pub fn evaluate(&self, x: &FieldElement<'a>) -> FieldElement {
        // Accumulate sum of terms multiplied by coefficients.
        self.coefficients
            .iter()
            .fold(
                (
                    FieldElement::zero(self.prime),
                    FieldElement::one(self.prime),
                ),
                |(sum, term), coeff| {
                    let sum = sum + coeff * &term;
                    let term = &term * x;
                    (sum, term)
                },
            )
            .0
    }
}

impl<'a> Add<Polynomial<'a>> for Polynomial<'a> {
    type Output = Polynomial<'a>;

    fn add(self, other: Polynomial<'a>) -> Self::Output {
        assert_eq!(self.prime, other.prime);

        // Add the coefficients of the two polynomials.
        let coefficients = self
            .coefficients
            .iter()
            .zip_longest(other.coefficients.iter())
            .map(|pair| match pair {
                Both(a, b) => a + b,
                Left(a) => a.clone(),
                Right(b) => b.clone(),
            })
            .collect();

        Polynomial::new(coefficients, self.prime)
    }
}

impl<'a> Mul<Polynomial<'a>> for Polynomial<'a> {
    type Output = Polynomial<'a>;

    fn mul(self, other: Polynomial<'a>) -> Self::Output {
        assert_eq!(self.prime, other.prime);

        // Initialize the coefficients to the sum of the lengths of the two polynomials.
        let mut new_coefficients = vec![
            FieldElement::zero(self.prime);
            self.coefficients.len() + other.coefficients.len() - 1
        ];

        // Multiply the coefficients of the two polynomials.
        for (i, coeff_self) in self.coefficients.iter().enumerate() {
            for (j, coeff_other) in other.coefficients.iter().enumerate() {
                new_coefficients[i + j] += coeff_self * coeff_other;
            }
        }

        Polynomial::new(new_coefficients, self.prime)
    }
}

impl<'a> Mul<&FieldElement<'a>> for Polynomial<'a> {
    type Output = Polynomial<'a>;

    fn mul(self, element: &FieldElement<'a>) -> Self::Output {
        let new_coefficients = self
            .coefficients
            .iter()
            .map(|coeff| coeff * element)
            .collect();
        Polynomial::new(new_coefficients, self.prime)
    }
}
