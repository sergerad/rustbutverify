#[derive(Debug)]
pub struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub fn is_mirror(left: Option<Box<Node>>, right: Option<Box<Node>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            left.value == right.value
                && is_mirror(left.left, right.right)
                && is_mirror(left.right, right.left)
        }
        _ => false,
    }
}

pub fn is_symmetric(root: Option<Box<Node>>) -> bool {
    match root {
        None => true,
        Some(node) => is_mirror(node.left, node.right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn symm() {
        let root = Some(Box::new(Node {
            value: 1,
            left: Some(Box::new(Node {
                value: 2,
                left: Some(Box::new(Node {
                    value: 3,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Node {
                value: 2,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 3,
                    left: None,
                    right: None,
                })),
            })),
        }));
        let is_symmetric = is_symmetric(root);
        assert!(is_symmetric);
    }
}
