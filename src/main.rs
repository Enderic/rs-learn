use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    // i32 is data, RefCell<Rc<List>> is practically "next"
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil)))); // We create an Rc<List> of a

    println!("a initial rc count = {}", Rc::strong_count(&a)); // There's 1 reference to a, so it prints 1
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // Create Rc<List> holding another List by cloning a

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // We have another reference to a's data on heap, so 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // For b, just the initialization so just 1
    println!("b next item = {:?}", b.tail()); // It goes b->a->Nil

    if let Some(link) = a.tail() {
        // When we get here, we first dereference the reference, then we do borrow_mut() to get the Rc<List>
        // Then we set the next to b, turning it into a cycle a <=> b
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}