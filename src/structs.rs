// named field structs
struct Computer {
    ram: i32,
    harddrive: i32,
    cpu_type: String,
}

// Syntax for creating an object:
fn make_computer1() -> Computer {
    Computer {
        ram: 4,
        harddrive: 1024,
        cpu_type: "Intel Core i7 7600".to_string(),
    }
}

// Struct creation are expressions too:
fn make_computer2() -> Computer {
    Computer {
        ram: 4,
        harddrive: 1024,
        cpu_type: "Intel Core i7 7600".to_string(),
    }
}

// Struct creation are expressions too:
fn make_computer3(cpu_type: String) -> Computer {
    // Variables already in scope somewhere:
    let ram = 4;
    let harddrive = 1024;
    // ...

    // No need for field: EXPR syntax.
    Computer {
        ram,
        harddrive,
        cpu_type,
    }
}

// Tuple like structs:
struct Pair(i32, i32);

fn pair() -> Pair {
    Pair(3, 3)
}

// Mainly used to wrap types.
// Using type X = Y; only creates a type synonym or alias, these types are exactly the
// same and interchangeable, if we don't want this, we can create to uniquely different
// types using:
struct SortedVector(Vec<i32>);

// Unit like struct:
struct Modified; // No members...

// In memory, values are stack allocated, just like C++ or C. This avoids heap allocation
// of values (which is what happen in Python, Java, etc).

// How would our struct Computer look in memory?
// No guarantee of ordering of fields, and remember there is padding!

// Use #[repr(C)] for identical C implementation.

// Methods
impl SortedVector {
    // Associated function.
    // Basically a "static" function in OO terminology.

    // nothing special about the word "new" just a convention.
    fn new(mut vec: Vec<i32>) -> SortedVector {
        vec.sort_unstable();
        SortedVector(vec)
    }

    fn push(&mut self, val: i32) {
        // Must explicitly use self to refer to fields in Sorted Vector
        self.0.push(val);
        self.0.sort_unstable();
    }

    fn split(self) -> (SortedVector, SortedVector) {
        unimplemented!();
    }
}

fn do_sorted_stuff() {
    let mut sv = SortedVector::new(vec![1, 2, 3]);
    // ...
    sv.push(3);

    let (sv1, sv2) = sv.split();

    // What happens?
    // sv.push(3);
}

// Functions not defined in any impl block are free functions.

// Talk about interior mutability?

// Save for next lecture on traits and generics?
// Better Definition of SortedVector:

// struct SortedVector2<T>{
//     vec: Vec<T>
// }

// impl<T: Ord> SortedVector2<T> {
//     fn new(mut vec: Vec<T>) -> Self {
//         vec.sort_unstable();
//         SortedVector2{vec}
//     }
// }
