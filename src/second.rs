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

#[allow(dead_code)]
/*
    Tuple structs - trivial wrappers around other types without having to name each field   
*/
pub struct IntoIter<T>(List<T>);

impl <T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next (&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
    /*
        Iter contains a reference to something, we need a lifetime specifier to ensure that reference lasts as long as needed
        Iter is generic over *some* lifetime, it doesn't care
    */
}

//No life time is needed on List because it doesn't have any associated lifetimes
impl <T> List<T> {
    /*
        A lifetime is declared here for the *exact* borrow that creates the Iter. 
        self (the List creating the Iter) needs to be valid for as long as Iter is around.
    */
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        /*
            Input expects an Option to the Node, however, we have an Option containing a pointer (Box) to the Node!
            we need to dereference (*) the pointer, however, we cannot return a reference to data owned locally!
                - recall map() moves the data!! It takes ownership.

            Hence we need to use as_ref to get a reference to the node, however, as_ref adds another layer of indirection! 
                - we would typically need to dereference the extra indirection, 
                  but Rust helps us with this with the as_deref() function, dereferencing the extra pointer
        */
        Iter { next: self.root.as_deref().map(|node| { &*node })}
    }
}

// A lifetime needs to be defined here because Iter has one that needs to be defined
impl <'a, T> Iterator for Iter<'a, T> {
    //lifetime needed here too, this is a type declaration
    type Item = &'a T;
    //code here does not need change due to Self::Item
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref().map(|node| &*node);
            &node.elem
        })
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

    #[test]
    pub fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}