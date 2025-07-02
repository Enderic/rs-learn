/*
    We can use the newtype pattern in a lot of ways
    In the previous section we had the Millimeter & Meter structs that wrapped a u32 in a newtype (tuple struct)
    If we try writing a function that takes a Millimeter type, program wouldn't compile if we tried to use Meters or u32

    Rust has type aliases where we can give a name for an existing type w/ the `type` keyword
    Treated the same as the the original type
    Only downside is we don't get type checking between the two, so if we mix them up, we don't get an error
*/

type Kilometers = i32;

// Main use case is to reduce repetition of long types
// Say we have the function below, we might wanna use that type in a lot of places, which can get confusing & repetitive
fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}

// If we put it in a type alias, then all we need to do is use Thunk, making everything more manageable
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_short_type(f: Thunk) {}

// We can also use it w/ Result<T, E> which is a common pattern that can get annoying to type out fully
// We can use ? on this as well like we do w/ regular Result types since it's treated as the same
type Res<T> = std::result::Result<T, std::io::Error>;

pub trait Writing {
    fn write(&mut self, buf: &[u8]) -> Res<usize>;
    fn flush(&mut self) -> Res<()>;
}


fn main() {

    let x: Kilometers = 6;
    let mut y = 1;

    // Can add together
    let mut z = x + y;
    println!("x + y = {}", x + y);

    // Pretty much treated like the same variable
    let mut a = &mut y;
    *a += 1;
    a = &mut z;
    *a = *a + x;
    println!("y, z ({}, {})", y, z);
}