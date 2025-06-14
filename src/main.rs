/*
    With Rc<T>, we can create weak references by calling Rc::downgrade(), which returns smart pointer Weak<T>
        - Weak references don't express ownership
        - Is kept track of by weak_count (doesn't need to be 0 for cleanup)

    Because the value Weak<T> references might've been dropped, we use Rc::upgrade() which returns an Option<Rc<T>>
*/

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    /*
        We want the node to contain its children nodes (Node)
        We want the children node to have ownership shared, like if we want a node to have multiple parents (Rc<Node>)
        We can have multiple children (Vec<Rc<Node>>)
        We want to be able to mutate even on immutable Nodes (RefCell<Vec<Rc<Node>>>)
    */
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // branch knows about leaf, but leaf doesn't know about branch, which we want to happen
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}