// When creating unsafe functions, they look exactly like regular functions but w/ the unsafe keyword before the definition
// With the unsafe keyword, we're responsible for what happens in the function & we need to handle everythin gin there
// We can only call unsafe functions in unsafe blocks
unsafe fn dangeous() {}

/*
    Say we wanna make a function that takes an array along w/ a midpoint & returns a two-tuple with two parts of the array split at the midpoint
    If we tried to make it in safe rust, it'll error, it would look something like this

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        assert!(mid <= len);
        (&mut values[..mid], &mut values[mid..]) <- Error here because we're borrow from same slice twice; can't understand we're borrowing different parts of it
    }

    When we know code is okay but Rust doesn't, we can use unsafe rust
*/

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // We get the length & raw pointer w/ .as_mut_ptr() of the slice
    let len = values.len();
    let ptr = values.as_mut_ptr();

    // Make sure the mid isn't bigger than the length
    assert!(mid <= len);

    // Use std::slice::from_raw_parts_mut(), which is unsafe, to create a slice
    // First we pass in the slice itself, which is the beginning, & the mid to show how far it goes from the start
    // Then we pass the slice + mid to get to the point, then make it go the rest of the slice w/ len - mid
    // The function is unsafe bc it uses a raw pointer & needs to trust the pointer is valid
    // the .add() function is also unsafe b/c it needs to trust the offset from the pointer is valid
    // Because of the previous two statements, we wrap it in an unsafe block
    // We can use it in safe code b/c the unsafe code is actually safe; only valid pointers are created
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    unsafe {
        dangeous();
    }

    /*
        The code below will fail b/c we try to create a huge slice from memory we don't own

        let address = 0x01234usize;
        let r = address as *mut i32;

        let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    */
}

