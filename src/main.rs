/*
    Orphan Rule - We can only implement a trait on a type if either the trait or type are local to the crate
    We can get around this using the Newtype Pattern (originates from Haskell)

    We wrap an existing type that we want to implement on in a tuple struct (thin wrapper)
    The wrapper struct will then be local to the crate & we can implement a trait on it
    
    If we try to implement Display on Vec<T>, the orphan rule prevents us from doing so since they're both not local to the crate
    We thin wrap Vec<T> in a tuple struct & implement Display on that wrapper struct
*/

use std::fmt;

// We would need to implement all the methods of Vec<String> in the Wrapper so we don't have to continuously call self.0 & treat Wrapper exactly like Vec<T>
// We can use the Deref trait on Wrapper to return the inner type to access every method of the inner type
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We use self.0 since Wrapper is a tuple struct, & 0 is the first element in that tuple which is the Vec<String>
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}