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
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // Just created leaf, so S-1, W-0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        /*
            We create a new Rc<NodeE> branch, it takes a clone of leaf, so leaf: S-2, W-0 & branch: S-1, W-0
            We mutably borrow leaf's parent & assign it to a Weak<> branch, so branch: S-1, W-1
        */
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    /*
        Branch goes out of scope & branch S-0, so it gets dropped
        Because of branch dropping, the Rc::clone() of leaf also gets dropped, so leaf: S-1, W-0
        Rust automatically returns None for us since branch was dropped in the inner scope
    */
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}