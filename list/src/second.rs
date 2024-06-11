pub struct List {
    pub head: Link,
}

type Link = Option<Box<Node>>;

pub struct Node {
    pub elem: i32,
    pub next: Link,
}

impl std::default::Default for List {
    fn default() -> Self {
        List { head: None }
    }
}

impl List {
    pub fn push(&mut self, elem: i32) {
        // Create a new head which points to the old head
        let new_head = Box::new(Node {
            elem,
            // Take the old head (move by replace, b/c &mut self)
            next: self.head.take(),
        });
        // Set the new head
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            // Set current head to old's next and return value
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // Take head
        let mut link = self.head.take();

        // Take all nodes
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}
