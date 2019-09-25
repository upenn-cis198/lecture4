// Simplest form of erroring: Panic:
// unrecoverable errors: out of bounds accesses, division by zero, calling unwrap on None,
// assertion failure.

fn my_panic(){
    // ... stuff
    panic!("Error description here"); // Works with format as well.
    // panic!("Error: {}", x);

    // Try not to panic.
}

// Panic unwind the stack, dropping variables in reverse order from declaration.
// Cleans up resources! This is a good thing!

// This is not a crash, it's well defined. Similar to Java Runtime Exception.



