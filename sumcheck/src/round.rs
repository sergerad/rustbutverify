use std::fmt::{Display, Formatter};

use crate::polynomial::{pretty_field, pretty_univariate, FieldElement, Univariate};

/// Represents the result of a round of the sum-check protocol.
#[derive(Debug, Clone)]
pub struct Round {
    /// The number from 1 to v that represents the current round of the protocol.
    /// Where v is the number of terms in the polynomial f.
    pub number: usize,

    /// The random field element r_i that is sent from the verifier to the prover.
    pub r_i: Option<FieldElement>,

    /// The univariate polynomial g_i that was used in this round.
    pub g_i: Option<Univariate>,

    /// The final evaluation of the polynomial f(r_1,...,r_v) == g_v(r_v).
    pub final_eval: Option<FieldElement>,
}

impl Display for Round {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.g_i {
            Some(g) => write!(
                f,
                "Round {}:\tg_{} = {}\tr_{} = {} ",
                self.number,
                self.number,
                pretty_univariate(g),
                self.number,
                pretty_field(&self.r_i.unwrap_or_default()),
            ),
            None => write!(
                f,
                "Finalized:\tg_{}(r_{})={}",
                self.number - 1,
                self.number - 1,
                pretty_field(&self.final_eval.unwrap())
            ),
        }
    }
}
