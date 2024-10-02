use std::iter::Sum;

use ark_bls12_381::Fr as ScalarField;
use ark_ff::Field;
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;
use rand::seq::index;
use rand::Rng;

pub use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
pub use ark_poly::polynomial::{MVPolynomial, Polynomial};

pub type Multivariate = SparsePolynomial<ScalarField, SparseTerm>;
pub type Univariate = UniSparsePolynomial<ScalarField>;

#[derive(Debug)]
pub struct SumCheck {
    /// The multivariate polynomial f that represents the arithmetic circuit being proven.
    f: Multivariate,

    /// The sum of f over the Boolean hypercube.
    sum: ScalarField,

    /// The univariate polynomials g_i that are derived from f.
    g: Vec<Univariate>,

    /// The random values r_i that are sent from the verifier to the prover.
    r: Vec<ScalarField>,

    /// The current round of the protocol.
    round: usize,
}

impl SumCheck {
    pub fn new(f: Multivariate) -> Self {
        // Calculate sum of f over Boolean hypercube
        let sum = Self::sum(&f);
        Self {
            f,
            sum,
            g: Vec::new(),
            r: Vec::new(),
            round: 1,
        }
    }

    fn sum(f: &Multivariate) -> ScalarField {
        let num_vars = f.num_vars();
        let hypercube_size = 2_usize.pow(num_vars.try_into().expect("f has too many variables"));
        (0..hypercube_size)
            .map(|i| f.evaluate(&hypercube_index_to_tuple(i, num_vars)))
            .sum()
    }
}

/// Creates a tuple of field elements from an index into a Boolean hypercube.
fn hypercube_index_to_tuple(index: usize, num_vars: usize) -> Vec<ScalarField> {
    (0..num_vars)
        .rev()
        .map(|shift| {
            let boolean = (index >> shift) & 1;
            (boolean as u8).into()
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Round {
    /// The number from 1-v that represents the current round of the protocol.
    /// Where v is the number of terms in the polynomial f.
    number: usize,

    /// The random value r_i that is sent from the verifier to the prover.
    r: ScalarField,

    /// The univariate polynomial g_i that was used in this round.
    g: Univariate,

    /// Whether the univariate g is of correct degree and evaluates to the expected result.
    is_valid: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hypercube_index_to_tuple() {
        let index = 0b00101;
        let expected: Vec<ScalarField> = vec![
            ScalarField::from(0),
            ScalarField::from(0),
            ScalarField::from(1),
            ScalarField::from(0),
            ScalarField::from(1),
        ];
        assert_eq!(hypercube_index_to_tuple(index, 5), expected);
    }
}
