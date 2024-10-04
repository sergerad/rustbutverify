use ark_poly::multivariate::Term;
use thiserror::Error as ThisError;

use ark_bls12_381::Fr as FieldElement;

use crate::polynomial::{
    derive_univariate, sum_multivariate, Error as PolynomialError, MVPolynomial, Multivariate,
    Polynomial, SparseTerm, Univariate,
};

pub use crate::round::Round;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("invalid sum for g_{0}(0) + g_{0}(1)")]
    Sum(usize),

    #[error("invalid degree for polynomial g_{0}: expected {1}, got {2}")]
    Degree(usize, usize, usize),

    #[error("invalid polynomial")]
    Polynomial(#[from] PolynomialError),
}

/// Representation of the sum-check protocol.
/// Produces a sum of a multivariate polynomial over the Boolean hypercube.
/// Repeats rounds for each variable in f until the following equation can
/// be verified: g_v(r_v) = f(r_1,...,r_v).
///
/// Intended to be used as an iterator that produces a [Round] for each iteration
/// of the sum-check algorithm.
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

    /// Flag to indicate if any rounds have failed.
    failed: bool,
}

impl SumCheck {
    /// Initializes a new instance of the [SumCheck] protocol.
    pub fn new(f: Multivariate) -> Result<Self, Error> {
        // Calculate sum of f over Boolean hypercube.
        let sum = sum_multivariate(&f)?;
        // Return initialized instance.
        Ok(Self {
            f,
            sum,
            g: Vec::new(),
            r: Vec::new(),
            round: 0,
            failed: false,
        })
    }

    /// Executes the first round of the sum-check protocol.
    /// Derives the univariate polynomial g_1 from f and verifies that
    /// S = g_1(0) + g_1(1).
    fn first_round(&mut self) -> Result<Round, Error> {
        // Derive univariate polynomial g_1 from f.
        let g_1 = derive_univariate(&self.f, &self.r);

        // Verify that g_1 is of correct degree.
        let term = &self.f.terms[0].1;
        if !Self::verify_degree(&g_1, term) {
            return Err(Error::Degree(self.round, g_1.degree(), term.degree()));
        }
        // Verify that g_1 evaluates to the expected result.
        let sum = g_1.evaluate(&0u32.into()) + g_1.evaluate(&1u32.into());
        if sum != self.sum {
            return Err(Error::Sum(self.round));
        }
        self.g.push(g_1.clone());

        let r_i = self.next_random();
        Ok(Round {
            number: self.round,
            r_i: Some(r_i),
            g_i: Some(g_1),
            final_eval: None,
        })
    }

    /// Executes the ith round of the sum-check protocol.
    /// Derives the univariate polynomial g_i from f and verifies that
    /// g_{i-1}(r_{i-1}) = g_i(0) + g_i(1).
    fn ith_round(&mut self) -> Result<Round, Error> {
        // Derive univariate polynomial g_i from f.
        let g_i = derive_univariate(&self.f, &self.r);

        // Verify that g_1 is of correct degree.
        let term = &self.f.terms[self.round - 1].1;
        if !Self::verify_degree(&g_i, term) {
            return Err(Error::Degree(self.round, g_i.degree(), term.degree()));
        }
        // Check that g_i(0) + g_i(1) = g_{i-1}(r_{i-1}).
        let sum_i = g_i.evaluate(&0u32.into()) + g_i.evaluate(&1u32.into());
        let sum_r = self.g[self.round - 2].evaluate(&self.r[self.round - 2]);
        if sum_i != sum_r {
            return Err(Error::Sum(self.round));
        }
        self.g.push(g_i.clone());

        let r_i = self.next_random();
        Ok(Round {
            number: self.round,
            r_i: Some(r_i),
            g_i: Some(g_i),
            final_eval: None,
        })
    }

    /// Executes the final round of the sum-check protocol.
    fn final_round(&mut self) -> Result<Round, Error> {
        // Check that g_v(r_v) = f(r_1,...,r_v).
        let g_i_1 = self.g[self.round - 2].clone();
        let r_i_1 = self.r[self.round - 2];
        let sum_g = g_i_1.evaluate(&r_i_1);
        let sum_f = self.f.evaluate(&self.r);
        if sum_g != sum_f {
            return Err(Error::Sum(self.round));
        }

        Ok(Round {
            number: self.round,
            r_i: None,
            g_i: None,
            final_eval: Some(sum_g),
        })
    }

    /// Checks that the total degree of the univariate polynomial is equal to the degree of the term.
    fn verify_degree(u: &Univariate, term: &SparseTerm) -> bool {
        u.degree() <= term.degree()
    }

    /// Creates a random field element and adds it to the list of random values.
    fn next_random(&mut self) -> FieldElement {
        // NOTE: We hard-code the random values in order to match the result from
        // the Sum-Check Protocol article from sergerad.xyz
        let r_i = FieldElement::from(4 - self.round as u64);
        self.r.push(r_i);
        r_i
    }

    /// Increments the round counter.
    fn start_round(&mut self) {
        self.round += 1;
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
        self.start_round();
        let round = match self.round {
            round if round > self.f.num_vars() + 1 => None,
            round if round == self.f.num_vars() + 1 => Some(self.final_round()),
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
