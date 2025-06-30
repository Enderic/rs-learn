// You can define a trait as unsafe if the compiler can't verify all safety guarantees
unsafe trait MyTrait {
    // method signatures here
}

// Implementing an unsafe trait requires `unsafe impl`
// You're taking manual responsibility for upholding invariants
unsafe impl MyTrait for i32 {
    // method implementations
}

// If we have a type A that has a type B that doesn't implement Send or Sync, like raw pointers, & we want to mark type A as Send or Sync, we must use unsafe
// Rust doesn't know if type B upholds safety rules regarding read/write from multiple threads, so we do it manually w/ unsafe