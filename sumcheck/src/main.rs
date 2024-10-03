mod sumcheck;
use sumcheck::{MVPolynomial, Multivariate, SparsePolynomial, SparseTerm, Term};

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

    // Initialize sumcheck instance against polynomial f.
    let sumcheck = sumcheck::SumCheck::new(f);

    // Execute the protcol round by round.
    for round in sumcheck.into_iter() {
        println!("{:?}", round?);
    }

    Ok(())
    // Calculate sum of f over Boolean hypercube

    // Pass sum S to verifier

    // Derive univariate polynomial g_1 from f and pass to V

    // V checks g_1 degree, and checks that g_1(0) + g_1(1) = S

    // V selects random r_1 and sends to P

    // P  derives g_2 and sends to V

    // V checks g_2 degree, and checks that g_2(0) + g_2(1) = g_1(r_1)

    // When round == number of terms in f, V checks that g_n(r_n) = f(r_1,...,r_n)
}
