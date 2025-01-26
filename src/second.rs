#[allow(dead_code)]
pub struct List {
    root: Link,
}

type Link = Option<Box<Node>>;
/*
    previous implementation of Link, is just a bad reimplementation of Option!
    replace implementation of Link with a type alias to Option<Box<Node>>
*/

#[allow(dead_code)]
struct Node {
    elem: i32,
    next: Link,
}

use std::mem;
impl List {
    pub fn new() -> List {
        List { root: Link::None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem, 
            next: self.root.take()
            /*
                Usage of mem::replace is so common, that Option makes it a method take()
            */
        });
        self.root = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.root.take().map(|node| {
            self.root = node.next;
            node.elem
        })
        /*
            This match pattern on an optional is a common idiom called map. 
            Map will take the value in Some(x) to produce a value of Some(y)
        */
    }
}


impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.root.take();
        while let Link::Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check None list behaves right
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