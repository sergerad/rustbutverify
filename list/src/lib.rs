pub mod first;
pub mod second;

#[cfg(test)]
mod test {
    trait List {
        fn push(&mut self, elem: i32);
        fn pop(&mut self) -> Option<i32>;
    }
    impl List for super::first::List {
        fn push(&mut self, elem: i32) {
            self.push(elem);
        }
        fn pop(&mut self) -> Option<i32> {
            self.pop()
        }
    }
    impl List for super::second::List {
        fn push(&mut self, elem: i32) {
            self.push(elem);
        }
        fn pop(&mut self) -> Option<i32> {
            self.pop()
        }
    }

    #[test]
    fn basics() {
        let tests: Vec<Box<dyn List>> = vec![
            Box::<super::first::List>::default(),
            Box::<super::second::List>::default(),
        ];
        for mut list in tests {
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
}
