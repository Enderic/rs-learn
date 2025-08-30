use std::sync::Mutex;
use std::thread;

fn main() {
    let (input, expected) = (&[
        "   ",
        " * ",
        "   ",
    ], &[
        "111",
        "1*1",
        "111",
    ]);

    let mut plants: Vec<(u16, u16)> = vec![];

    for (r, row) in input.iter().enumerate() {
        for (i, plant) in row.char_indices() {
            if plant == '*' {
                
            }
        }
    }

    println!("{plants:?}");
}

fn set_amounts() {
    // x,y => plant
    // 
}