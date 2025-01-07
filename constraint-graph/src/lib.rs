pub mod builder;
pub mod graph;

pub use builder::Builder;
pub use graph::Graph;

#[cfg(test)]
mod test {
    use crate::builder::Builder;
    use parameterized::parameterized;

    #[parameterized(x_val = {1, 2, 3}, y_val = {7, 11, 17})]
    fn add_mul(x_val: u32, y_val: u32) {
        // f(x) = x^2 + x + 5
        let mut builder = Builder::default();
        let x = builder.init();
        let x_squared = builder.mul(&x, &x);
        let five = builder.constant(5);
        let x_squared_plus_5 = builder.add(&x_squared, &five);
        let y = builder.add(&x_squared_plus_5, &x);

        // Evaluate and check constraints
        let mut graph = builder.fill(&[x_val]);
        assert!(graph.check_constraints(y, y_val));
    }

    #[parameterized(x_val = {1, 2, 3})]
    fn add_mul_equality(x_val: u32) {
        // f(x) = x^2 + x + 5
        let mut builder = Builder::default();
        let x = builder.init();
        let x_squared = builder.mul(&x, &x);
        let five = builder.constant(5);
        let x_squared_plus_5 = builder.add(&x_squared, &five);
        let y = builder.add(&x_squared_plus_5, &x);
        let yy = builder.add(&x_squared_plus_5, &x);
        let y_equal_yy = builder.assert_equal(&y, &yy);

        // Evaluate and check constraints
        let mut graph = builder.fill(&[x_val]);
        assert!(graph.check_constraints(y_equal_yy, 0));
    }

    #[parameterized(a_val = {7, 17, 63}, c_val = {1, 2, 8})]
    fn mul_hint(a_val: u32, c_val: u32) {
        // function f(a):
        //     b = a + 1
        //     c = b / 8
        //     return c
        let mut builder = Builder::default();
        let a = builder.init();
        let one = builder.constant(1);
        let b = builder.add(&a, &one);
        let c = builder.hint(&b, |b_value| b_value / 8);

        // Evaluate and check constraints
        let mut graph = builder.fill(&[a_val]);
        assert!(graph.check_constraints(c, c_val));
    }

    #[parameterized(x_val = {2, 9, 57})]
    fn sqrt_hint(x_val: u32) {
        // f(x) = sqrt(x+7)
        let mut builder = Builder::default();
        let x = builder.init();
        let seven = builder.constant(7);
        let x_plus_seven = builder.add(&x, &seven);

        let sqrt_x_plus_7 = builder.hint(&x_plus_seven, |x_plus_seven| {
            (x_plus_seven as f32).sqrt() as u32
        });
        let computed_sq = builder.mul(&sqrt_x_plus_7, &sqrt_x_plus_7);
        let eq = builder.assert_equal(&computed_sq, &x_plus_seven);

        // Evaluate and check constraints
        let mut graph = builder.fill(&[x_val]);
        assert!(graph.check_constraints(eq, 0));
    }
}
