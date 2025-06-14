/*
    With Rc<T>, we can create weak references by calling Rc::downgrade(), which returns smart pointer Weak<T>
        - Weak references don't express ownership (doesn't determine when value drops)
        - Is kept track of by weak_count instead of strong_count (doesn't need to be 0 for cleanup)

    Because the value Weak<T> references might've been dropped, we use Rc::upgrade() which returns an Option<Rc<T>>
*/

use std::cell::RefCell;
use std::rc::{Rc, Weak};

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
    /*
        We want our child to be aware of the parent
        It can't be Rc<T> because this will cause a reference cycle like before
        A child node should be dropped if the parent node is dropped, but not the other way around
        For that second scenario, Weak<T> is the best choice
        We use RefCell<T> to mutate it even if its immutable
    */
    parent: RefCell<Weak<Node>>,
}

fn main() {
    // Create leaf (bottom) node without any parent
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // Printing leaf node parent results to None 
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // Create branch node with no parent
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("{} - {}", Rc::strong_count(&leaf), Rc::strong_count(&branch));
    /*
        First we take leaf then dereference it out of the Rc<>
        Get the parent then take a mutable borrow (interior mutability w/ RefCell<>)
        Set the (now) mutable parent property to a Weak<> ptr that contains branch
    */
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // Now when we print out leaf's parent, it shows the actual parent we assigned it to
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Here, leaf strong_count is 2 & branch strong_count is 1
    // When leaf drops, its strong_count only goes down by 1, leaving it at 1
    // When branch drops, its strong_count goes to 0, so everything within it, including the Rc::clone(&leaf) is dropped as well,
    // making leaf strong_count go to 0 as well, which makes both get dropped properly
}