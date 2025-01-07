use std::{collections::HashMap, rc::Rc};

/// An identifier for a [Node] in the [Graph].
/// Used to map input values to variable [Node]s in the [Graph].
pub type Id = usize;

/// A hint function that takes a computed value of some [Node] and returns
/// some resulting value.
pub type Hint = fn(u32) -> u32;

/// A node in the [Graph] representing a variable, constant, operation, or hint.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Node {
    /// A variable node in the [Graph].
    /// Its value is determined by the input values to the [Graph].
    Variable(Id),
    /// A constant value node in the [Graph].
    Constant(u32),
    /// A node in the [Graph] representing a binary operation.
    Operation {
        operator: Operator,
        left: Rc<Node>,
        right: Rc<Node>,
    },
    /// A node in the [Graph] representing a hint.
    /// Similar to a preocmpile, it is not computed in the [Graph] directly.
    Hint(Rc<Node>),
}

/// The different operators that can be used in the graph.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Operator {
    /// Add two [Node]s together.
    Add,
    /// Multiply two [Node]s together.
    Multiply,
    /// Check if two [Node]s are equal (a-b=0).
    Equality,
}

/// A graph of [Node]s representing a mathematical function.
/// Contains computed evaluations of [Node]s in the graph.
/// Can be used to check arithmetic constraints on the graph.
#[derive(Debug)]
pub struct Graph {
    pub(crate) evaluations: HashMap<Node, u32>,
}

impl Graph {
    /// Checks whether the result of a [Node] matches the expected value.
    pub fn check_constraints(&mut self, result: Node, expected_value: u32) -> bool {
        let result = self.evaluations.get(&result).unwrap();
        *result == expected_value
    }
}
