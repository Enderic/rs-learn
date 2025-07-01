/*
    Rust doesn't stop us from having two traits w/ the same method names OR implementing those to traits on one type
    We can even go as far as implementing a method on the type w/ the same name as the trait function
*/

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Here we have non-method functions; w/o a self parameter
trait Animal {
    fn baby_name() -> String;
}

trait Fat {
    fn fat_name() -> String {
        String::from("Fat")
    }
}

struct Dog;

impl Dog {
    // This is NOT a method since it doesn't have `self`
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    let person = Human;
    person.fly(); // When we call it like this, we get the directly implementation of .fly() from Human
    // We can call specific traits of the function using types by specifying trait name then passing the object through, this works b/c .fly() takes the self parameter
    Pilot::fly(&person);
    Wizard::fly(&person);

    // This uses the direct implementation on Dog 
    println!("A baby dog is called a {}", Dog::baby_name());
    // For using the Animal impl of .baby_name(), we can't just do ` Animal::baby_name()` since Rust won't know which implementation to use
    // We use fully qualified syntax to call which implementation specifically to call
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // Syntax goes like: <Type as Trait>::function(receiver_if_method, next_arg, ...);

    // Even though there's a default implementation for this function in the trait, we still need to specify the specific implementeation
    //println!("{}", Fat::fat_name());
}