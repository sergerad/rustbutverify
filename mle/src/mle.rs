use crate::{field::FieldElement, multivariate::Multivariate};

/// Computes the multilinear extension of a multivariate function at a point in Fp.
pub fn multilinear_extension<const P: u128>(
    multivariate: Multivariate<P>,
    point: Vec<FieldElement<P>>,
) -> FieldElement<P> {
    let one = FieldElement::<P>::one();
    // Sum the product of the multilinear Lagrange basis polynomials weighted by the evaluations.
    let sum = multivariate
        .into_iter()
        .map(|(tuple, eval)| {
            // Compute the product of multilinear Lagrange basis polynomials.
            let prod = tuple
                .iter()
                .by_vals()
                .rev()
                .zip(point.iter())
                .map(|(bit, r)| {
                    let one_or_zero = FieldElement::<P>::from(bit);
                    *r * one_or_zero + (one - *r) * (one - one_or_zero)
                })
                .product();
            // Weight the product by the evaluation of boolean tuple from the multivariate function.
            eval * prod
        })
        .sum();
    sum
}
