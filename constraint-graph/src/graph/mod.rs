use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
    Divide,
    Equality,
}

type Id = usize;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Node {
    Variable(Id),
    Constant(u32),
    Operation {
        operator: Operator,
        left: Rc<Node>,
        right: Rc<Node>,
    },
}

#[derive(Debug)]
pub struct Builder {
    nodes: Vec<Rc<Node>>,
}

impl Builder {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
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

    pub fn hint(&mut self, node: Node) -> Self {
        todo!()
    }

    pub fn fill(self, inputs: &[u32]) -> Graph {
        Graph {
            evaluations: HashMap::new(),
            nodes: self.nodes,
            inputs: inputs.to_vec(),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Rc<Node>>,
    inputs: Vec<u32>,
    evaluations: HashMap<Node, u32>,
}

impl Graph {
    pub fn check_constraints(&mut self, result: Node, expected_value: u32) -> bool {
        for node in self.nodes.iter() {
            match &**node {
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
                    Operator::Divide => {
                        todo!()
                    }
                    Operator::Equality => {
                        todo!()
                    }
                },
            }
        }
        let result = self.evaluations.get(&result).unwrap();
        *result == expected_value
    }
}
