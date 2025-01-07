# Constraint Graph

This crate contains a lib for constructing graphs that represent arithmetic circuits.

The `Builder` struct can be used to construct graphs out of mathematical functions. The `Graph` struct is created by the `Builder` (type-state). The `Graph` can be used to test constraints defined in the graph.

```rust
use constraint_graph::{Builder, Graph};

/// Construct a graph for f(x) = x^2
let mut builder = Builder::default();
let x = builder.init();
let x_squared = builder.mul(&x, &x);

/// We can create an equality constraint
let nine = builder.constant(9);
let eq = builder.assert_equal(&x_squared, &nine);

/// Evaluate the graph with a particular input
let mut graph = builder.fill(&[3]);

/// We check the node directly
assert!(graph.check_constraints(x_squared, 9));

/// We can check the equality constraint
assert!(graph.check_constraints(eq, 0));
```

To test the crate, run the following from the repository root dir:

```sh
cargo test -p constraint-graph
```
