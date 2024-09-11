use std::fmt::Debug;

use bitvec::vec::BitVec;

use crate::{field::FieldElement, hypercube::Hypercube};

/// Represents a precomputed multivariate function over a prime field.
/// Maps v-bit tuples to Fp.
#[derive(Clone)]
pub struct Multivariate<const P: u128> {
    evaluations: Vec<(BitVec, FieldElement<P>)>,
}

impl<const P: u128> Multivariate<P> {
    /// Constructs a new [`Multivariate`] function and precomputes the evaluations.
    pub fn new(hypercube: Hypercube, f: impl Fn(&BitVec) -> FieldElement<P>) -> Self {
        let evaluations = hypercube
            .into_iter()
            .map(|tuple| {
                let eval = f(&tuple);
                (tuple, eval)
            })
            .collect::<Vec<_>>();
        Self { evaluations }
    }
}

impl Debug for Multivariate<5> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (tuple, eval) in &self.evaluations {
            writeln!(
                f,
                "f{:?} -> {:?}",
                tuple
                    .iter()
                    .by_vals()
                    .map(|bit| {
                        if bit {
                            1
                        } else {
                            0
                        }
                    })
                    .rev()
                    .collect::<Vec<_>>(),
                eval
            )?;
        }
        Ok(())
    }
}

impl<const P: u128> IntoIterator for Multivariate<P> {
    type Item = (BitVec, FieldElement<P>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.evaluations.into_iter()
    }
}
