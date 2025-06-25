/*
    Unsafe rust gives us many kinds of functionality
        - Dereference a raw pointer
        - Call an unsafe method/function
        - Access/modify a mutable static variable
        - Implement an unsafe treait
        - Access fields of a 'union'

    Unsafe rust doesn't turn off any safety checks, there's still some degree of safety in unsafe rust
    It's our job to ensure code in unsafe rust works properly
*/
fn main() {
    /*
        In unsafe rust, we have raw pointers
        They can be immutable or mutable like references, written as `*const T` & `*mut T`
        - The * is a part of the type name
        With raw pointers specifically, immutable means that they can't be directly assigned after being dereferenced

        They're allowed to ignore borrow rules regarding having both immutable & mutable pointers/multiple mutable pointers on the same location
        Aren't guaranteed to point to valid memory
        Can be null
        Doesn't implement automatic cleanup
    */

    // We can create raw pointers in safe code, but can't dereference them
    // Notice how we can have an immutable & mutable raw pointer to the same thing, unlike w/ references
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    // We can't always assume raw pointers are valid
    let address = 0x012345usize; // We create a pointer to raw memory location
    let r = address as *const i32; // We use 'as' to cast to an immutable raw pointer

    // We now need an unsafe block to dereference the pointers to num
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}