/*
    Sometimes an owner might have multiple owners, like when multiple edges in a graph point to same vertex
    We enable multiple ownership by using Rc<T> (Reference Counting), which keeps track of the numer of references
        - If there's 0 references, value is cleaned up
    Only for use in single-threaded scenarios
    Also for read-only (immutable references)
*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use crate::List::{Cons, Nil};

fn main() {
    /*
        Assume we're using Box<T> instad of Rc<T> for this scenario

        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a));
        let c = Cons(4, Box::new(a));
        
        This errors b/c Cons own data they hold, so when b is created, a is moved into b, & c can't use it
        We could specify lifetime parameters to say that every element inthe list lives as long as the entire list, but this isn't alwasy true
    */

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); 
    let b = Cons(3, Rc::clone(&a)); // We do a clone of the Rc<List> a (normal .clone() also works)
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        // When c goes out of scope, the count decreases
        // The Drop trait does the decrease for us
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}