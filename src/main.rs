/*
    When using generic type parameters, we can specify a default concrete type for it
    In this case, we implement the Add trait to our Point struct to add two points together
*/

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    // Still need to explicitly define the type of the parameter
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// This thin wrapping of an existing type (Meters/Millimeters wrapping over u32) is called `newtype pattern`
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    f
}

// Add implementation
// Rhs=Self; Rhs is a generic & Self is the default concrete type
// This is saying that the default type of Rhs will be the type we're implementing Add on
// In the point implementation, we do use the default for Rhs
// In the meters implementation, we add millimeters w/ meters, not using the default for Rhs
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}