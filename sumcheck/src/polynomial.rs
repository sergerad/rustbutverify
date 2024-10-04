use ark_ff::Field;
use ark_ff::PrimeField;
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;
use thiserror::Error as ThisError;

pub use ark_bls12_381::Fr as FieldElement;
pub use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
pub use ark_poly::polynomial::{MVPolynomial, Polynomial};

pub type Multivariate = SparsePolynomial<FieldElement, SparseTerm>;
pub type Univariate = UniSparsePolynomial<FieldElement>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("invalid multivariate polynomial: {0}")]
    InvalidMultivariate(&'static str),
}

/// Calculates the sum of the polynomial f over the Boolean hypercube.
pub fn sum_multivariate(f: &Multivariate) -> Result<FieldElement, Error> {
    let num_vars = f.num_vars();
    let hypercube_size = 2_usize.pow(
        num_vars
            .try_into()
            .map_err(|_| Error::InvalidMultivariate("f has too many variables"))?,
    );
    let sum = (0..hypercube_size)
        .map(|i| {
            // Convert index into Boolean hypercube to tuple of field elements in {0,1}^v.
            let tuple = index_to_boolean_tuple(i, num_vars);
            // Evaluate f at tuple and sum the results.
            f.evaluate(&tuple)
        })
        .sum();
    Ok(sum)
}

/// Derives the univariate polynomial g_i from the polynomial f based on the number of
/// r_i's in r and their values. The univariate polynomial g_i is a linear combination
/// of evaluations of f at tuples in the hypercube containing a single fixed variable.
///
/// For example, if f = a + 2b^2 + 3ac^3, then its evaluations are:
/// f(a,0,0) + f(a,0,1) + f(a,1,0) + f(a,1,1) = 4 + 10a
pub fn derive_univariate(f: &Multivariate, r: &[FieldElement]) -> Univariate {
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
            sum + evaluate_multivariate(f, r, &tuple)
        },
    )
}

/// Evaluates the polynomial f at a tuple with a single fixed variable.
/// For example, evaluates f(a,0,0) = a, where f = a + 2b^2 + 3ac^3.
pub fn evaluate_multivariate(
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
            let (eval, univariate_term) = evaluate_multivariate_term(r, term, tuple);
            // Linear combination.
            let u =
                Univariate::from_coefficients_vec(vec![(univariate_term.degree(), *coeff * eval)]);
            u + sum
        },
    )
}

/// Evaluates a term of the polynomial f at a tuple with a single fixed variable,
/// some Booleans, and optionally some r_i's.
/// For example, the term 3ac^3 with (r_1,r_2,c) would be ealuated to 3r_1c^3 for r = [r_1,r_2].
/// The locations of r_i's and the fixed variable in the tuple are determined by the length of r.
pub fn evaluate_multivariate_term(
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

/// Creates a {0,1}^v tuple that corresponds to an integer-based index.
/// For example, 6 = [1,1,0].
pub fn index_to_boolean_tuple(index: usize, num_vars: usize) -> Vec<FieldElement> {
    (0..num_vars)
        .rev()
        .map(|shift| {
            let boolean = (index >> shift) & 1;
            (boolean as u8).into()
        })
        .collect()
}

/// Pretty prints a field element.
///
/// # Panics
///
/// Will panic for any field elements that are greater than 2^64.
/// Only intended to be used with small values for learning purposes.
pub fn pretty_field(e: &FieldElement) -> i64 {
    let e = e.into_repr().to_string();
    i64::from_str_radix(&e, 16).unwrap()
}

/// Pretty prints a univariate polynomial.
///
/// # Panics
///
/// Will panic for any field elements that are greater than 2^64.
/// Only intended to be used with small values for learning purposes.
pub fn pretty_univariate(u: &Univariate) -> String {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_index_to_tuple() {
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
    fn multivariate_sum() {
        let f: Multivariate = SparsePolynomial::from_coefficients_vec(
            3,
            vec![
                (1u32.into(), SparseTerm::new(vec![(0, 1)])), // a
                (2u32.into(), SparseTerm::new(vec![(1, 2)])), // 2b^2
                (3u32.into(), SparseTerm::new(vec![(0, 1), (2, 3)])), // 3ac^3
            ],
        );
        let sum = sum_multivariate(&f).unwrap();
        assert_eq!(sum, 18u32.into());
    }

    #[test]
    fn univariate_from_multivariate() {
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
        let g = derive_univariate(&f, &r);
        let expected = Univariate::from_coefficients_vec(vec![(0, 4u32.into()), (1, 10u32.into())]);
        assert_eq!(g, expected);

        // g_2
        let mut r = vec![FieldElement::from(3)];
        let g = derive_univariate(&f, &r);
        let expected = Univariate::from_coefficients_vec(vec![(0, 15u32.into()), (2, 4u32.into())]);
        assert_eq!(g, expected);

        // g_3
        r.push(FieldElement::from(2));
        let g = derive_univariate(&f, &r);
        let expected = Univariate::from_coefficients_vec(vec![(0, 11u32.into()), (3, 9u32.into())]);
        assert_eq!(g, expected);
    }
}
