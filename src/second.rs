#[allow(dead_code)]
pub struct List <T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;
/*
    previous implementation of Link, is just a bad reimplementation of Option!
    replace implementation of Link with a type alias to Option<Box<Node>>
*/

#[allow(dead_code)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

use std::mem;
impl <T> List <T> {
    pub fn new() -> List<T> {
        List { root: Link::None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem, 
            next: self.root.take()
            /*
                Usage of mem::replace is so common, that Option makes it a method take()
            */
        });
        self.root = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.root.take().map(|node| {
            self.root = node.next;
            node.elem
        })
        /*
            This match pattern on an optional is a common idiom called map. 
            Map will take the value in Some(x) to produce a value of Some(y)
        */
    }

    pub fn peek(&self) -> Option<&T> {
        /*
            map() takes self by-value, consuming the original value. 
            as_ref creates an Option to a reference to the value inside the original, for map to take
        */
        self.root.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.root.as_mut().map(|node| {
            &mut node.elem
        })
    }
}


impl <T> Drop for List<T> {
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        // list.peek_mut().map(|&mut value| {
        //     value = 42
        // });
        /*
            The above looks like value should be declared as mutable reference, however, 
            for closures, it specifies a pattern that will be matched against arguments to the closure.
            |&mut value| means "the argument is a mutable reference, but just copy it into value"
        */
        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}