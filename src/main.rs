/*
    One detail Rust needs to know about types is how much space to allocate for a value of that type
    This sort of conflicts w/ dynamically sized types/unsized types (DST) whos size isn't known on compile time

    A str on its own is a DST b/c we don't know how long it is until runtime, so we can't create a str variable OR have an argument take a str
    We know to use &str but why?

    A &str is two values: The address of the str & length
    Since we have those two values, it's twice the size of a usize, which is a known size
        - We call this a fat pointer

    The actual data is can be stored on the stack, heap, or binary, &str just points there
        - The binary is another region of memory just like the stack or heap, & &str points to that
        - main() has three variables, s1 is a &str pointing to data on heap & s2 is a &str pointing to data in binary
        - s3 is utilizing Box<str>
    
    The golden rule of DSTs is we should put values of DSTs behind a pointer of some kind

    Traits are also DSTs, & that's why in function parameters we declare &dyn Trait or Box<dyn Trait>, since those have known sizes
        - Reminder: dyn Trait is a trait object, & the size of it isn't known on compile time, & this gives us dynamic dispatch
        - We can use generics to make a function of that generic function for every type that implements the trait, giving static dispatch
        - Example is given below

    Rust gives us the Sized trait to determine if a type's size is known at compile time, auto-implemented for types w/ sizes known at compile time
*/

// Using generics for static dispatch vs using dyn/trait object for dynamic dispatch
fn test<T: std::fmt::Display>(a: T) {}
fn test_two(a: &dyn std::fmt::Display) {}

// Here's a generic function
fn generic<T>(t: T) {}

// Rust implicitly assumes every generic in a generic function to be Sized
// This only works on types that have a known size at compile time
fn generic_imp<T: Sized>(t: T) {}

// We can use this special syntax to relax the restriction from above tho
// In this case, ?Sized means T may or may not be sized
// The ? syntax ONLY works w/ Sized
// We use &T since the type needs to be behind some pointer since the size might not be known
fn generic_fat<T: ?Sized>(t: &T) {}

fn main() {
    // The below will error
    // If we're allowed to write this, we would need to allocate two str values but of different lengths, but they need to take up the same amount of space
    //let s1: str = "Hello there!";
    //let s2: str = "How's it going?";

    let s1: &str = &String::from("fat");
    let s2: &str = "fat";

    // Can't use `Box::new(String::from("Fat"))` since this is a Box<String> which is different from a Box<str>
    // We use .into_boxed_str() to turn into a Box<str>
    let s3: Box<str> = String::from("Fat").into_boxed_str();
    
    println!("s1, s2, s3: {}, {}, {}", s1, s2, s3);
}