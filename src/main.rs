/*
    Drop trait lets you customize what happens when a value is gonna go out of scope
        - Box<T> deallocates space on heap when it's dropped
*/

use std::mem::drop;
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c); // Can't do c.drop()
    println!("C CSP was dropped.");
    // When c & d go out of scope (without the drop function doing anything), they go in reverse order of their initialization because they're on the stack like in c++
    // W/ the drop of c, c will drop before d now
}