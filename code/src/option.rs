// Rust has options types, an enumeration which is either Some(val) or None:

fn take_option(val: Option<i32>){
    match val {
        Some(x) => println!("Option contained: {}", x),
        None => println!("Nothing in option"),
    }

    // Similarish to:
    println!("Option: {:?}", val);
}

// Useful for taking optional values. Can represent functions which may only
// return a value sometimes:
// Rough type of get function for HashTable:

// fn get(&self, k: &Q) -> Option<&V>

// If element is missing returns none.

// Rust does not have exceptions.
// Options must be explicitly matched: compiler enforces this, no more null pointer
// exceptions :)

// In Java similar method may look like:

// If value is missing returns Null
// V get(Q k)
