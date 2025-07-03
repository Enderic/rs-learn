/*
    We can pass in regular functions into other functions
    Functions have the type fn (not Fn closure trait), which is a function pointer
    We can use function pointers as arguments for other functions

    fn is a type, not a trait like Fn
    fn implement all hree closure traits: Fn, FnMut, & FnOnce
        - Can always pass a fn as an argument for an argument that expects a closure
        - One case where we would only want a fn & not a closure is when interfacing w/ external code that doesn't have closures, like C
*/

// We define a function
fn add_one(x: i32) -> i32 {
    x + 1
}

// Each variant in an enum becomes an initializer function that we can use as function pointers taht implement closure traits
enum Status {
    Value(u32),
    Stop,
}

// The syntax is fn(parameter types) -> return type
// In this case, it's an fn that takes one i32 as input & returns an i32
// We call it twice passing in arg & return sum
// Syntax for function parameters similar to closures
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    // Using the function & passing in the function pointer
    let answer = do_twice(add_one, 3);
    println!("The answer is {answer}");

    let list_of_numbers = vec![1, 2, 3];

    // Doing the same thing w/ a closure & iterator
    let list_of_strings_cl: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Need fully qualified syntax since there's many .to_string() functions
    let list_of_strings_fn: Vec<String> = 
        list_of_numbers.iter().map(ToString::to_string).collect();

    // We create a vector of Status::Value instances using a range, then calling .map() on it & use the initializer function, then collect all of it into a vector
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}