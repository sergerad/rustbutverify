use crate::{field::FieldElement, multivariate::Multivariate};

/// Computes the multilinear extension of a multivariate function at an element x in Fp.
pub fn multilinear_extension<const P: u128>(
    multivariate: Multivariate<P>,
    x: Vec<FieldElement<P>>,
) -> FieldElement<P> {
    let one = FieldElement::<P>::one();
    // Sum the product of the multilinear Lagrange basis polynomials weighted by the evaluations.
    let sum = multivariate
        .into_iter()
        .map(|(tuple, eval)| {
            // Compute the product of multilinear Lagrange basis polynomials inerpolated by w.
            let w = tuple.iter().by_vals().rev();
            let product = w
                .zip(x.iter())
                .map(|(wi, &xi)| {
                    // x_i * w_i + (1 - x_i)(1 - w_i)
                    let wi = FieldElement::<P>::from(wi);
                    xi * wi + (one - xi) * (one - wi)
                })
                .product();
            // Weight the product by the evaluation of boolean tuple from the multivariate function.
            eval * product
        })
        .sum();
    sum
}
