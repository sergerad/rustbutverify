mod polynomial;
mod round;
mod sumcheck;

use polynomial::{MVPolynomial, Multivariate, SparsePolynomial, SparseTerm, Term};
use sumcheck::SumCheck;

fn main() -> Result<(), sumcheck::Error> {
    // Define a polynomial f that represents an arithmetic circuit.
    let f: Multivariate = SparsePolynomial::from_coefficients_vec(
        // a + 2b^2 + 3ac^3
        3,
        vec![
            (1u32.into(), SparseTerm::new(vec![(0, 1)])), // a
            (2u32.into(), SparseTerm::new(vec![(1, 2)])), // 2b^2
            (3u32.into(), SparseTerm::new(vec![(0, 1), (2, 3)])), // 3ac^3
        ],
    );
    println!("Defined:\tf   = a + 2b^2 + 3ac^3");

    // Initialize sumcheck instance against polynomial f.
    let sumcheck = SumCheck::new(f)?;

    // Execute the protocol round by round.
    for round in sumcheck.into_iter() {
        println!("{}", round?);
    }

    Ok(())
}
