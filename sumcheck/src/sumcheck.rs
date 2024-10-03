use core::num;
use std::iter::Sum;
use thiserror::Error as ThisError;

use ark_bls12_381::Fr as FieldElement;
use ark_ff::PrimeField;
use ark_ff::{Field, UniformRand};
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;
use rand::seq::index;
use rand::Rng;

pub use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
pub use ark_poly::polynomial::{MVPolynomial, Polynomial};

pub type Multivariate = SparsePolynomial<FieldElement, SparseTerm>;
pub type Univariate = UniSparsePolynomial<FieldElement>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("invalid sum for g_{0}(0) + g_{0}(1)")]
    InvalidSum(usize),
}

#[derive(Debug)]
pub struct SumCheck {
    /// The multivariate polynomial f that represents the arithmetic circuit being proven.
    f: Multivariate,

    /// The sum of f over the Boolean hypercube.
    sum: FieldElement,

    /// The univariate polynomials g_i that are derived from f.
    g: Vec<Univariate>,

    /// The random values r_i that are sent from the verifier to the prover.
    r: Vec<FieldElement>,

    /// The current round of the protocol.
    round: usize,

    failed: bool,
}

impl SumCheck {
    /// Initializes a new instance of the [SumCheck] protocol.
    pub fn new(f: Multivariate) -> Self {
        // Calculate sum of f over Boolean hypercube.
        let sum = Self::sum(&f);
        Self {
            f,
            sum,
            g: Vec::new(),
            r: Vec::new(),
            round: 1,
            failed: false,
        }
    }

    fn first_round(&mut self) -> Result<Round, Error> {
        // Derive univariate polynomial g_1 from f.
        let g_1 = Self::derive_univariate(&self.f, &self.r);

        // Verify that g_1 is of correct degree and evaluates to the expected result.
        let s = g_1.evaluate(&0u32.into()) + g_1.evaluate(&1u32.into());
        if s != self.sum {
            println!("s: {:?}, sum: {:?}", s, self.sum);
            return Err(Error::InvalidSum(self.round));
        }

        // Increment round.
        self.round += 1;

        Ok(Round {
            number: self.round - 1,
            r: None,
            g: Some(g_1),
        })
    }

    fn ith_round(&mut self) -> Result<Round, Error> {
        let mut rng = rand::thread_rng();
        let r_i = FieldElement::rand(&mut rng);
        self.r.push(r_i);
        let g_i = Self::derive_univariate(&self.f, &self.r);
        self.round += 1;

        Ok(Round {
            number: self.round - 1,
            r: Some(r_i),
            g: Some(g_i),
        })
    }

    fn final_round(&mut self) -> Result<Round, Error> {
        self.round += 1;
        //self.r.push(r_i);

        Ok(Round {
            number: self.round - 1,
            r: None,
            g: None,
        })
    }

    /// Calculates the sum of the polynomial f over the Boolean hypercube.
    fn sum(f: &Multivariate) -> FieldElement {
        let num_vars = f.num_vars();
        let hypercube_size = 2_usize.pow(num_vars.try_into().expect("f has too many variables"));
        (0..hypercube_size)
            .map(|i| {
                // Convert index into Boolean hypercube to tuple of field elements in {0,1}^v.
                let tuple = index_to_boolean_tuple(i, num_vars);
                // Evaluate f at tuple and sum the results.
                f.evaluate(&tuple)
            })
            .sum()
    }

    /// Derives the univariate polynomial g_i from the polynomial f based on the number of
    /// r_i's in r and their values. The univariate polynomial g_i is a linear combination
    /// of evaluations of f at tuples in the hypercube containing a single fixed variable.
    ///
    /// For example, if f = a + 2b^2 + 3ac^3, then its evaluations are:
    /// f(a,0,0) + f(a,0,1) + f(a,1,0) + f(a,1,1) = 4 + 10a
    fn derive_univariate(f: &Multivariate, r: &[FieldElement]) -> Univariate {
        // The permutations of the Boolean hypercube are now {0,1}^(v-r-1).
        let num_vars = f.num_vars() - r.len() - 1;
        let size = 2_usize.pow((num_vars).try_into().expect("f has too many variables"));

        // Iterate over the permutations of the Boolean hypercube and evaluate f at each tuple.
        (0..size).fold(
            Univariate::from_coefficients_vec(vec![(0, 0u32.into())]),
            |sum, n| {
                // Create a {0,1}^(v-r-1) tuple and prepend a 1 to represent the fixed variable.
                let tuple = index_to_boolean_tuple(n, num_vars);
                let tuple = vec![FieldElement::from(1)]
                    .into_iter()
                    .chain(tuple)
                    .collect::<Vec<_>>();
                // Derived univariate is a linear combination of evaluations of f over the tuples.
                sum + Self::evaluate_multivariate(f, r, &tuple)
            },
        )
    }

    /// Evaluates the polynomial f at a tuple with a single fixed variable.
    /// For example, evaluates f(a,0,0) = a, where f = a + 2b^2 + 3ac^3.
    fn evaluate_multivariate(
        f: &Multivariate,
        r: &[FieldElement],
        tuple: &[FieldElement],
    ) -> Univariate {
        // Evaluate every term of f at the tuple and sum the results.
        f.terms().iter().fold(
            Univariate::from_coefficients_vec(vec![]),
            |sum, (coeff, term)| {
                // Evaluate the term of the polynomial f at the tuple.
                // The results could be a constant or a univariate polynomial.
                let (eval, univariate_term) = Self::evaluate_multivariate_term(r, term, tuple);
                // Linear combination.
                let curr = Univariate::from_coefficients_vec(vec![(
                    univariate_term.degree(),
                    *coeff * eval,
                )]);
                curr + sum
            },
        )
    }

    /// Evaluates a term of the polynomial f at a tuple with a single fixed variable,
    /// some Booleans, and optionally some r_i's.
    /// For example, the term 3ac^3 with (r_1,r_2,c) would be ealuated to 3r_1c^3 for r = [r_1,r_2].
    /// The locations of r_i's and the fixed variable in the tuple are determined by the length of r.
    fn evaluate_multivariate_term(
        r: &[FieldElement],     // E.G. [r_1,r_2]
        term: &SparseTerm,      // E.G. 3ac^3
        tuple: &[FieldElement], // E.G. (r_1,r_2,c), or (a,0,0), etc
    ) -> (FieldElement, SparseTerm) {
        // The univariate term corresponds to the fixed variable in the tuple.
        // It may or may not exist in this term.
        let mut univariate_term: SparseTerm = SparseTerm::new(vec![]);
        // Evaluate every variable in ther multivariate term.
        let eval: FieldElement = term
            .iter()
            .fold(1u32.into(), |product, (var, pow)| match *var {
                // This is the fixed E.G. a in f(a,0,0). It is not evaluated.
                var if var == r.len() => {
                    univariate_term = SparseTerm::new(vec![(var, *pow)]);
                    product
                }
                // This is r_i in E.G. f(r_1,b,0). Read r and evaluate.
                var if var < r.len() => {
                    let r_i = r[var];
                    r_i.pow([*pow as u64]) * product
                }
                // Evaluate the variable based on the corresponding element from the tuple.
                _ => tuple[*var - r.len()].pow([*pow as u64]) * product,
            });
        // For the fixed variable, evaluation == 1, and the univariate term is not empty.
        // For all other variables, evaluation == r_i or tuple_i, and univariate term is empty.
        (eval, univariate_term)
    }
}

impl Iterator for SumCheck {
    type Item = Result<Round, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // Bail out if any iterations have failed.
        if self.failed {
            return None;
        }

        // Execute the relevant round.
        let round = match self.round {
            round if round > self.f.num_vars() => None,
            round if round == self.f.num_vars() => Some(self.final_round()),
            1 => Some(self.first_round()),
            _ => Some(self.ith_round()),
        };

        // Check if the iteration failed.
        round.as_ref().inspect(|round| {
            if !round.is_ok() {
                self.failed = true;
            }
        });

        // Return the result.
        round
    }
}

/// Creates a {0,1}^v tuple of field elements from an integer-based index.
fn index_to_boolean_tuple(index: usize, num_vars: usize) -> Vec<FieldElement> {
    (0..num_vars)
        .rev()
        .map(|shift| {
            let boolean = (index >> shift) & 1;
            (boolean as u8).into()
        })
        .collect()
}

fn pretty_univariate(u: &Univariate) -> String {
    u.iter().fold("".to_string(), |acc, (i, c)| {
        let c = c.clone().into_repr().to_string();
        let c = i64::from_str_radix(&c, 16).unwrap();
        let prefix = if acc.is_empty() { "" } else { " + " };
        match i {
            0 => format!("{acc}{prefix}{c}"),
            1 => format!("{acc}{prefix}{c}x"),
            _ => format!("{acc}{prefix}{c}x^{i}"),
        }
    })
}

#[derive(Debug, Clone)]
pub struct Round {
    /// The number from 1-v that represents the current round of the protocol.
    /// Where v is the number of terms in the polynomial f.
    number: usize,

    /// The random field element r_i that is sent from the verifier to the prover.
    r: Option<FieldElement>,

    /// The univariate polynomial g_i that was used in this round.
    g: Option<Univariate>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn index_to_tuple() {
        let index = 0b00101;
        let expected: Vec<FieldElement> = vec![
            FieldElement::from(0),
            FieldElement::from(0),
            FieldElement::from(1),
            FieldElement::from(0),
            FieldElement::from(1),
        ];
        assert_eq!(index_to_boolean_tuple(index, 5), expected);
    }

    #[test]
    fn sum_multivariate() {
        let f: Multivariate = SparsePolynomial::from_coefficients_vec(
            3,
            vec![
                (1u32.into(), SparseTerm::new(vec![(0, 1)])), // a
                (2u32.into(), SparseTerm::new(vec![(1, 2)])), // 2b^2
                (3u32.into(), SparseTerm::new(vec![(0, 1), (2, 3)])), // 3ac^3
            ],
        );
        let sum = SumCheck::sum(&f);
        assert_eq!(sum, 18u32.into());
    }

    #[test]
    fn derive_univariate() {
        let f: Multivariate = SparsePolynomial::from_coefficients_vec(
            3,
            vec![
                (1u32.into(), SparseTerm::new(vec![(0, 1)])), // a
                (2u32.into(), SparseTerm::new(vec![(1, 2)])), // 2b^2
                (3u32.into(), SparseTerm::new(vec![(0, 1), (2, 3)])), // 3ac^3
            ],
        );
        // g_1
        let r = vec![];
        let g = SumCheck::derive_univariate(&f, &r);
        let expected = Univariate::from_coefficients_vec(vec![(0, 4u32.into()), (1, 10u32.into())]);
        let e = expected.evaluate(&FieldElement::from(1));
        println!("{}", pretty_univariate(&expected));
        println!("{}", pretty_univariate(&g));
        println!("{}", e.to_string());
        let e = g.evaluate(&FieldElement::from(1));
        println!("{}", e.to_string());
        assert_eq!(g, expected);

        // g_2
        let mut r = vec![FieldElement::from(3)];
        let g = SumCheck::derive_univariate(&f, &r);
        let expected = Univariate::from_coefficients_vec(vec![(0, 15u32.into()), (2, 4u32.into())]);
        println!("expected {}", pretty_univariate(&expected));
        println!("got      {}", pretty_univariate(&g));
        assert_eq!(g, expected);

        // g_3
        r.push(FieldElement::from(2));
        let g = SumCheck::derive_univariate(&f, &r);
        let expected = Univariate::from_coefficients_vec(vec![(0, 11u32.into()), (3, 9u32.into())]);
        println!("expected {}", pretty_univariate(&expected));
        println!("got      {}", pretty_univariate(&g));
        assert_eq!(g, expected);
    }
}
