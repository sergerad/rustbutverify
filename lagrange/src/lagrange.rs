use crate::polynomial::Polynomial;
use crate::FieldElement;
use num_bigint::BigInt;
use num_traits::{One, Zero};

// Computes the ith Lagrange basis polynomial for a vector of size n.
// The ith basis polynomial is the product of (x - k)/(i - k) for k != i over the prime field Fp.
fn compute_basis_polynomial(i: usize, n: usize, p: &BigInt) -> Polynomial {
    let mut basis = Polynomial::new(vec![FieldElement::new(BigInt::one(), p)], p);

    for k in 0..n {
        // For every input element apart from i, accumulate multiplication.
        if k != i {
            // (i - k)^(-1)
            let i = FieldElement::new(BigInt::from(i), p);
            let k = FieldElement::new(BigInt::from(k), p);
            let denominator = (i.clone() - k.clone()).inverse();

            // Represent the linear factor (x - k) as a polynomial.
            let factor = Polynomial::new(
                vec![
                    // Constant term, -k.
                    FieldElement::new(BigInt::zero() - (*k).clone(), p),
                    // Linear term, 1.
                    FieldElement::new(BigInt::one(), p),
                ],
                p,
            );

            // (x - k) * (i - k)^(-1)
            let kth = factor * &denominator;
            // Accumulate the product of all linear terms.
            basis = basis * kth;
        }
    }

    basis
}

// Given a set of evaluations, computes the Lagrange interpolation polynomial over the prime field Fp.
pub fn lagrange_interpolation<'a>(
    evaluations: Vec<FieldElement<'a>>,
    p: &'a BigInt,
) -> Polynomial<'a> {
    // Initialize the resulting Lagrange polynomial to zero.
    let mut l = Polynomial::new(vec![FieldElement::new(BigInt::zero(), p)], p);

    // Calculate the Lagrange interpolation polynomial.
    let n = evaluations.len();
    for (x, y) in evaluations.iter().enumerate() {
        // Calculate basis polynomial.
        let basis = compute_basis_polynomial(x, n, p);
        // Weight the basis polynomial by the corresponding evaluation.
        let weighted = basis * y;
        // Sum up the weighted basis polynomials.
        l = l + weighted;
    }

    l
}
