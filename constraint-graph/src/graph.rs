use std::{collections::HashMap, rc::Rc};

/// An identifier for a node in the graph.
/// Used to map input values to variables in the graph.
pub type Id = usize;

/// A hint function that takes a computed value of some node and returns
/// some resulting value.
pub type Hint = fn(u32) -> u32;

/// A node in the graph representing a variable, constant, operation, or hint.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Node {
    Variable(Id),
    Constant(u32),
    Operation {
        operator: Operator,
        left: Rc<Node>,
        right: Rc<Node>,
    },
    Hint(Rc<Node>),
}

/// The different operators that can be used in the graph.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
    Equality,
}

/// A graph of nodes representing a mathematical function.
#[derive(Debug)]
pub struct Graph {
    pub(crate) evaluations: HashMap<Node, u32>,
}

impl Graph {
    /// Checks whether the result of a node matches the expected value.
    pub fn check_constraints(&mut self, result: Node, expected_value: u32) -> bool {
        let result = self.evaluations.get(&result).unwrap();
        *result == expected_value
    }
}
