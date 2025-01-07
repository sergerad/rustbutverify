use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
    Equality,
}

type Id = usize;

type Hint = fn(u32) -> u32;

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

#[derive(Debug)]
pub struct Builder {
    nodes: Vec<Rc<Node>>,
    hints: HashMap<Node, Hint>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            hints: HashMap::new(),
        }
    }

    pub fn init(&mut self) -> Node {
        let node = Node::Variable(self.nodes.len());
        self.nodes.push(node.clone().into());
        node
    }

    pub fn constant(&mut self, value: u32) -> Node {
        let node = Node::Constant(value);
        self.nodes.push(node.clone().into());
        node
    }

    pub fn add(&mut self, left: &Node, right: &Node) -> Node {
        let node = Node::Operation {
            operator: Operator::Add,
            left: left.clone().into(),
            right: right.clone().into(),
        };
        self.nodes.push(node.clone().into());
        node
    }

    pub fn mul(&mut self, left: &Node, right: &Node) -> Node {
        let node = Node::Operation {
            operator: Operator::Multiply,
            left: left.clone().into(),
            right: right.clone().into(),
        };
        self.nodes.push(node.clone().into());
        node
    }

    pub fn assert_equal(&mut self, left: &Node, right: &Node) -> Node {
        let node = Node::Operation {
            operator: Operator::Equality,
            left: left.clone().into(),
            right: right.clone().into(),
        };
        self.nodes.push(node.clone().into());
        node
    }

    pub fn hint(&mut self, node: &Node, hint: Hint) -> Node {
        let node = Node::Hint(node.clone().into());
        self.hints.insert(node.clone(), hint);
        self.nodes.push(node.clone().into());
        node
    }

    pub fn fill(self, inputs: &[u32]) -> Graph {
        Graph {
            nodes: self.nodes,
            inputs: inputs.to_vec(),
            hints: self.hints,
            evaluations: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Rc<Node>>,
    inputs: Vec<u32>,
    hints: HashMap<Node, Hint>,
    evaluations: HashMap<Node, u32>,
}

impl Graph {
    pub fn check_constraints(&mut self, result: Node, expected_value: u32) -> bool {
        for node in self.nodes.iter() {
            match node.as_ref() {
                Node::Variable(id) => {
                    self.evaluations
                        .insert(node.as_ref().clone(), self.inputs[*id]);
                }
                Node::Constant(value) => {
                    self.evaluations.insert(node.as_ref().clone(), *value);
                }
                Node::Operation {
                    operator,
                    left,
                    right,
                } => match operator {
                    Operator::Add => {
                        let left = self.evaluations.get(left.as_ref()).unwrap();
                        let right = self.evaluations.get(right.as_ref()).unwrap();
                        self.evaluations.insert(node.as_ref().clone(), left + right);
                    }
                    Operator::Multiply => {
                        let left = self.evaluations.get(left.as_ref()).unwrap();
                        let right = self.evaluations.get(right.as_ref()).unwrap();
                        self.evaluations.insert(node.as_ref().clone(), left * right);
                    }
                    Operator::Equality => {
                        let left = self.evaluations.get(left.as_ref()).unwrap();
                        let right = self.evaluations.get(right.as_ref()).unwrap();
                        self.evaluations.insert(node.as_ref().clone(), left - right);
                    }
                },
                Node::Hint(other) => {
                    let hint = self.hints.get(node.as_ref()).unwrap();
                    let other_value = self.evaluations.get(other.as_ref()).unwrap();
                    let hint_value = hint(*other_value);
                    self.evaluations.insert(node.as_ref().clone(), hint_value);
                }
            }
        }
        let result = self.evaluations.get(&result).unwrap();
        *result == expected_value
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_mul() {
        // f(x) = x^2 + x + 5
        let mut builder = Builder::new();
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
        let mut builder = Builder::new();
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
        let mut builder = Builder::new();
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
        let mut builder = Builder::new();
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
