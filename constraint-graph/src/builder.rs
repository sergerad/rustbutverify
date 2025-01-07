use crate::graph::{Graph, Hint, Node, Operator};
use std::{collections::HashMap, rc::Rc};

/// Builds a [Graph] of [Node]s representing a mathematical function.
#[derive(Debug, Default)]
pub struct Builder {
    nodes: Vec<Rc<Node>>,
    hints: HashMap<Node, Hint>,
}

impl Builder {
    /// Initializes a new variable [Node] in the [Graph].
    pub fn init(&mut self) -> Node {
        let node = Node::Variable(self.nodes.len());
        self.nodes.push(node.clone().into());
        node
    }

    /// Initalizes a new constant [Node] in the [Graph].
    pub fn constant(&mut self, value: u32) -> Node {
        let node = Node::Constant(value);
        self.nodes.push(node.clone().into());
        node
    }

    /// Initializes a new addition operation [Node] in the [Graph].
    pub fn add(&mut self, left: &Node, right: &Node) -> Node {
        let node = Node::Operation {
            operator: Operator::Add,
            left: left.clone().into(),
            right: right.clone().into(),
        };
        self.nodes.push(node.clone().into());
        node
    }

    /// Initializes a new multiplication operation [Node] in the graph.
    pub fn mul(&mut self, left: &Node, right: &Node) -> Node {
        let node = Node::Operation {
            operator: Operator::Multiply,
            left: left.clone().into(),
            right: right.clone().into(),
        };
        self.nodes.push(node.clone().into());
        node
    }

    /// Initializes a new equality operation [Node] in the [Graph].
    pub fn assert_equal(&mut self, left: &Node, right: &Node) -> Node {
        let node = Node::Operation {
            operator: Operator::Equality,
            left: left.clone().into(),
            right: right.clone().into(),
        };
        self.nodes.push(node.clone().into());
        node
    }

    /// Initializes a new hint node in the graph.
    /// The evaluation of the node will be computed using the hint function.
    pub fn hint(&mut self, node: &Node, hint: Hint) -> Node {
        let node = Node::Hint(node.clone().into());
        self.hints.insert(node.clone(), hint);
        self.nodes.push(node.clone().into());
        node
    }

    /// Fills the [Graph] with input values and evaluates all [Node]s.
    /// The returned [Graph] can be used to check constraints of the computation.
    pub fn fill(self, inputs: &[u32]) -> Graph {
        self.evaluate(inputs)
    }

    /// Evaluates all [Node]s in the [Graph] using the provided input values.
    fn evaluate(self, inputs: &[u32]) -> Graph {
        let mut evaluations = HashMap::new();
        for node in self.nodes.into_iter() {
            match node.as_ref() {
                Node::Variable(id) => {
                    evaluations.insert(node.as_ref().clone(), inputs[*id]);
                }
                Node::Constant(value) => {
                    evaluations.insert(node.as_ref().clone(), *value);
                }
                Node::Operation {
                    operator,
                    left,
                    right,
                } => match operator {
                    Operator::Add => {
                        let left = evaluations.get(left.as_ref()).unwrap();
                        let right = evaluations.get(right.as_ref()).unwrap();
                        evaluations.insert(node.as_ref().clone(), left + right);
                    }
                    Operator::Multiply => {
                        let left = evaluations.get(left.as_ref()).unwrap();
                        let right = evaluations.get(right.as_ref()).unwrap();
                        evaluations.insert(node.as_ref().clone(), left * right);
                    }
                    Operator::Equality => {
                        let left = evaluations.get(left.as_ref()).unwrap();
                        let right = evaluations.get(right.as_ref()).unwrap();
                        evaluations
                            .insert(node.as_ref().clone(), left.checked_sub(*right).unwrap_or(1));
                    }
                },
                Node::Hint(other) => {
                    let hint = self.hints.get(node.as_ref()).unwrap();
                    let other_value = evaluations.get(other.as_ref()).unwrap();
                    let hint_value = hint(*other_value);
                    evaluations.insert(node.as_ref().clone(), hint_value);
                }
            }
        }
        Graph { evaluations }
    }
}
