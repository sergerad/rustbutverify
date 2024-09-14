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
            let bits = tuple.iter().by_vals().rev();
            let product = bits
                .zip(point.iter())
                .map(|(w, &x)| {
                    // x_i * w_i + (1 - x_i)(1 - w_i)
                    let w = FieldElement::<P>::from(w);
                    x * w + (one - x) * (one - w)
                })
                .product();
            // Weight the product by the evaluation of boolean tuple from the multivariate function.
            eval * product
        })
        .sum();
    sum
}
