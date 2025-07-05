/*
    We've used macros before like println! but don't really know what they are
    macro refers to a family of features:
    1. Declarative Macros
    2. Prodecural Macros
        - Custom `#[derive]` macros taht specify code added w/ `derive` attribute used on structs & enums
        - Attribute-like macros that define custom attributes usable on any item
        - Function-like macros that look like function calls but operate on the tokens specified as their argument

    Macros are a way of writing code that writes other code; metaprogramming
    The `derive` attribute generates implementation of various traits for you
    We also use println!() & vec!(), & all of these macros produce more code

    Metaprogramming useful for reducing the amount of code we have to write & maintain, like functions but here are some things only they can do
        - Macros can take on a variable number of parameters, like how println!() works
        - Expanded before the compiler interprets the code; When Rust sees it, they're expanded into actual code; code generators

    Some downsides are:
        - More complexity & harder to read
        - Must define before calling them anywhere
*/

/*
    The most widely used form of macros is the declarative macro (macros by example; `maro_rules!` macro)
    Their behavior is similar to a match expression, but for source code structure instead of values
    The macro takes Rust code as input, matches it against defined patterns, and generates corresponding replacement code.
    This all happens during compilation
    Defined using `macro_rules!` 
*/

// Slightly simplified definition of vec! macro
// The actual definition includes stuff to alocate the correct amount of memory up front
// Indicates macro should be made available whenver the crate where the macro is defined is brought into scope
#[macro_export]
// Start the definition w/ macro_rules! & name of macro (w/o !)
macro_rules! vec {
    // Notice how this structure is similar to a match expression
    // Valid pattern syntax in macros is different from what we've been using
    // Wrap the whole thing in ()
    // $() means to match smoeone 0 or more times
    // $x:expr matches any Rust expression & is given the name $x
    // , says that each instance that matches w/ the expression above must be separated by a comma
    // * indicates the pattern matches 0 or more than any number that's after *
    // When we call vec![1, 2, 3], the $x pattern matches 3 times w/ 3 expressions: 1, 2, & 3 
    ( $( $x:expr ),* ) => {
        {
            // We create a vector
            // $()* says for each expression that matches, we push the expression into the vector
            // Then we return the vector
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec

            // The code generated that replaces the macro is:
            /*
                {
                    let mut temp_vec = Vec::new();
                    temp_vec.push(1);
                    temp_vec.push(2);
                    temp_vec.push(3);
                    temp_vec
                }
            */
        }
    };
}

fn main() {
    // We use a macro to create a vector w/ 3 integers
    // We could also only do 2 integers, or 6 string slices, couldn't use a function to do the same
    let v: Vec<u32> = vec![1, 2, 3];
}