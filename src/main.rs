/*
    A recursive type is a type that contains itself (node for linked list). 
    Rust needs to know exactly how much space a type takes up, but recursive types COULD be infinity (as much as the machine can allocate), so Rust doesn't know how much space it needs
    Since Box<> have a known size, we can use those for the recursive type inside
*/

/* 
    Cons List is a type of linked list
    Each item/node contains the current item value & next item/node
    (1, nil) -> (1, (2, nil)) -> (1, (2, (3, Nil)))
        - Nil means the base case of a recursion, as null/nil doesn't exist in rust
        - Usually just better to use a Vec<T>

    This code alone won't compile at all since List doesn't have a known size
    enum List {
        Cons(i32, List),
        Nil,
    }

*/
#[allow(dead_code)]
use std::mem::size_of_val;

// Rust sees each type to see which type takes up the most & makes that the size of each enum type
// In main(), the size of different variants are the same

// When it does the same w/ the List struct, it checks the i32 & List of Cons, then it looks into the List & checks its contents as well, & this becomes an infinite loop
// We use a Box<> since a pointer size doesn't change no matter what it's pointing to
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {

    let a = Message::Move{x: 12, y: 24};
    let b = Message::Quit;

    println!("Size of Move is {}", size_of_val(&a));
    println!("Size of Quit is {}", size_of_val(&b));

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{}", reverse("ผู้เขียนโปรแกรม"));
    
}

fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}