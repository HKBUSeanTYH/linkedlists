/*
pub enum List {
    Empty,
    Elem(i32, Box<List>)
}
The above implementation is not good, because:
    1. the first node will not be allocated on heap.
    2. the tail node will consume space for a pointer and an element (wasted space!)
See the following:

    layout 1:

    [Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr) -> (Empty *junk*)

    split off C:

    [Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)
    [Elem C, ptr] -> (Empty *junk*)

We should instead have:
layout 2:

[ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)

split off C:

[ptr] -> (Elem A, ptr) -> (Elem B, *null*)
[ptr] -> (Elem C, *null*)
 */
#[allow(dead_code)]
pub struct List {
    root: Link,
}
#[allow(dead_code)]
enum Link {
    Empty,
    PointerTo(Box<Node>),
}
#[allow(dead_code)]
struct Node {
    elem: i32,
    next: Link,
}

/*
This implementation of List allows us to:
    1. never allocated extra node (junk) on tail of a list
    2. enum will be in null-pointer-optimized form
    3. all elements uniformly allocated on heap
 */

use std::mem;
impl List {
    pub fn new() -> List {
        List { root: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem, 
            next: mem::replace(&mut self.root, Link::Empty)
        });
        /*
            mem::replace
            This incredibly useful function lets us steal a value out of a borrow by replacing it with another value
         */
        self.root = Link::PointerTo(new_node);
    }
}

/*
    There are 3 primary forms that self can take: self, &mut self, and &self. These 3 forms represent the three primary forms of ownership in Rust:
        self - Value (represents true ownership)
        &mut self - mutable reference (represents temporary exclusive access to a value that you don't own)
        &self - shared reference (represents temporary shared access to a value that you don't own)
*/