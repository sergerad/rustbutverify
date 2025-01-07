pub mod builder;
pub mod graph;

#[cfg(test)]
mod test {
    use crate::builder::Builder;
    #[test]
    fn add_mul() {
        // f(x) = x^2 + x + 5
        let mut builder = Builder::default();
        let x = builder.init();
        let x_squared = builder.mul(&x, &x);
        let five = builder.constant(5);
        let x_squared_plus_5 = builder.add(&x_squared, &five);
        let y = builder.add(&x_squared_plus_5, &x);

        let mut graph = builder.fill(&[2]);
        assert!(graph.check_constraints(y, 11));
    }

    #[test]
    fn add_mul_equality() {
        // f(x) = x^2 + x + 5
        let mut builder = Builder::default();
        let x = builder.init();
        let x_squared = builder.mul(&x, &x);
        let five = builder.constant(5);
        let x_squared_plus_5 = builder.add(&x_squared, &five);
        let y = builder.add(&x_squared_plus_5, &x);
        let yy = builder.add(&x_squared_plus_5, &x);
        let y_equal_yy = builder.assert_equal(&y, &yy);

        let mut graph = builder.fill(&[2]);
        assert!(graph.check_constraints(y_equal_yy, 0));
    }

    #[test]
    fn mul_hint() {
        // function f(a):
        //     b = a + 1
        //     c = b / 8
        //     return c
        let mut builder = Builder::default();
        let a = builder.init();
        let one = builder.constant(1);
        let b = builder.add(&a, &one);
        let c = builder.hint(&b, |b_value| b_value / 8);

        let mut graph = builder.fill(&[7]);
        assert!(graph.check_constraints(c, 1));
    }

    #[test]
    fn sqrt_hint() {
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

        let mut graph = builder.fill(&[2]);
        assert!(graph.check_constraints(eq, 0));
    }
}
