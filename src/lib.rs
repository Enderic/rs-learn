/*
    This struct is used to hold content & the state of the content
    The state is a Box<dyn State> wrapped in an Option<>, so any type that implements State in a Box in an Option

    - We use `Box<dyn State>` because:
        - `dyn State` is a *trait object*, which means the compiler doesn't know its size at compile time
        - Trait objects are *unsized* types, so they must be stored behind a pointer like `Box`, `Rc`, or `&dyn`.
        - `Box` allocates the state on the heap and allows for *dynamic dispatch*, where the actual method implementation is selected at runtime based on the concrete type.
        - Remember that Box<> owns the data it holds inside

    - We wrap the `Box` in an `Option<>` because:
        - We need to *take ownership* of the state when transitioning between states.
        - `Option::take()` lets us move the `Box<dyn State>` out of the struct safely, replacing it with `None`.
        - Without `Option`, we couldn't move the state out without violating ownership rules.

    Together:
    - `Box<dyn State>` allows us to store any type that implements the `State` trait.
    - `Option` allows us to temporarily remove (and replace) the state during transitions like `request_review()` or `approve()`.

    This design enables the *state pattern* to be used safely and idiomatically in Rust.
*/
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        // We use .take() to take the value out of Some() & leave a None in place
        // If we didn't use an option before, we would have to do more just to clean up the original state
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// Below here are all the states the post can have
// Each state is an empty struct that implements the State trait
// By taking `Box<Self>`, each method consumes the current state, returning the new one
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // This method only applies when called on Box<> holding that type
    // The function takes ownership of self since we're not referencing self; helps w/ clean up of the previous state
    // It'll return a Box<PendingReview> & PendingReview also implements State
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {} 

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}