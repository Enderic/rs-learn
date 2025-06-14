/*
    What's commonly used is Rc<T> w/ RefCell<T>

    We take Rc<T> which can only hold multiple immutable accesses to data
        & pair it w/ RefCell<T> to be able to modify those immutable references

    
*/

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let test = 3;
    let r = &test;
    let value = Rc::new(RefCell::new(r));

    // We dereference the Rc<RefCell<>> to get the RefCell<>
    let cell = RefCell::new(1);

    *cell.borrow_mut() = 3;
}