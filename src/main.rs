/*
    Rust has the ! type that's called the never type since it has no value
    Below is a function that returns this never type
*/
fn bar() -> ! {
    panic!("This will always panic!")
}

// From the guessing game we had this code
// Each arm must return the same type, so if Err(_) arm returned string, it wouldn't compile
// This works because continue returns !, Rust looks at both types, & sees continue (!) & u32
// Since ! can't ever have a value, it says guess is a u32
// The ! type can work w/ any other type
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

// Implementation for Option<T>
// The never type is also useful w/ panic! macro as well
// In this case the same thing happens, the match sees two types, T & panic! which is a ! type, so the result of the match is of type T, making this work as well
impl<T> Optionn<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

// A loop also is also of type !
// Since the loop doesn't end, it's of type !, but if it did have a break or ends in some way, this wouldn't be the case
fn main() {
    print!("forever");

    loop {
        print!("and ever");
    }
}