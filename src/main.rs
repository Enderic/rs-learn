// We can also use extern to let other languages call rust functions
// On a function, we add `extern` & specify the ABI before `fn`
// We also add `[unsafe(no_mangle)]` to not mangle the name of the function
// Mangling is when compiler changes name of function to a different name
// Language compilers do this & differently per language, so we do this to disable it since there could e name collisions
// This is also unsafe so we have to mark it unsafe

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
