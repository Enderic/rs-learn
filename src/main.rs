/*
    Attribute-like macros are similar to derive macros but they let us create new attributes
    Works for structs, enums, & other items like functions

    Function-like macros are similar to macro_rules! & can take an unknown number of arguments
    macro_rules! can only be defined w/ match like syntax
    Function-like macros take a TokenStream & manipulate the TokenStream using Rust code like the other two macros do
*/

// Attribute-like Macro
#[route(GET, "/")]
fn index() {}

// Notice how we use proc_macro_attribute instead of derive this time to signify an attribute macro
// The first parameter is for the contents of the attribute, in this case `GET, "/"` is for attr
// The item is for the body of the item the attribute is attached to, so in this case `fn index() {}`
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// Function-like Macro
// This would parse an SQL statment inside & check it's correct which is way more than what a macro_rules! can do
let sql = sql!(SELECT * FROM posts WHERE id=1);

// The definition would look something like this
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}