pub struct List {
    pub head: Link,
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    pub elem: i32,
    pub next: Link,
}

impl std::default::Default for List {
    fn default() -> Self {
        List { head: Link::Empty }
    }
}

impl List {
    pub fn push(&mut self, elem: i32) {
        // Create a new head which points to the old head
        let new_head = Box::new(Node {
            elem,
            // Take the old head (move by replace, b/c &mut self)
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        // Set the new head
        self.head = Link::More(new_head);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Move the old head out
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            // Set current head to old's next and return value
            Link::More(old_head) => {
                self.head = old_head.next;
                Some(old_head.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // Take head
        let mut link = std::mem::replace(&mut self.head, Link::Empty);

        // Take all nodes
        while let Link::More(mut node) = link {
            link = std::mem::replace(&mut node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::default();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
