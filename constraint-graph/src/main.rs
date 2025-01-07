mod graph;

// TODO: Parse polynomial string from command line args.
fn main() {
    // f(x) = x^2 + x + 5
    let mut builder = graph::Builder::new();
    let x = builder.init();
    let x_squared = builder.mul(&x, &x);
    let five = builder.constant(5);
    let x_squared_plus_5 = builder.add(&x_squared, &five);
    let y = builder.add(&x_squared_plus_5, &x);

    let mut graph = builder.fill(&[2]);
    let constrained = graph.check_constraints(y, 11);
    println!("{:?}", graph);
    println!("{:?}", constrained);
}
