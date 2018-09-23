use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Result;
use std::io::Read;
use std::error::Error;

fn read_whole_file(file: &str) -> String {
    // pub fn open<P: AsRef<Path>>(path: P) -> Result<File>
    let fd = File::open(file);
    let our_file = match fd {
        Ok(our_file) => our_file,
        Err(e) => panic!("Unable to open file: {}", e),
    };

    let reader = BufReader::new(our_file);
    let mut string = String::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => panic!("Failed to read line: {}", e),
        };

        string.push_str(& line);
    }

    string
}

fn read_whole_file_decent(file: &str) -> Result<String> {
    let fd = File::open(file)?;

    let reader = BufReader::new(fd);
    let mut string = String::new();
    for line in reader.lines() {
        string.push_str(& line?);
    }

    Ok(string)
}

fn read_whole_file_good(file: &str) -> Result<String> {
    let mut string = String::new();
    let file = File::open(file)?;
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut string)?;
    Ok(string)
}

// People used to write this kind of code.
fn our_main() {
    match run(){
        Ok(_) => {},
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<()> {
    read_whole_file_good("foo.txt")?;
    // ...
    read_whole_file_good("foo2.txt")?;

    Ok(())
}

// Newer versions of rust let you simply have main() return a result:
fn main() -> Result<()> {
    read_whole_file_good("foo.txt")?;
    // ...
    read_whole_file_good("foo2.txt")?;

    Ok(())
}

// Handling errors of multiple error types.

// Trait object can hold any type which implements some trait. For this,
// it is the Error trait.
fn f() -> Box<dyn Error> {
    unimplemented!();
}

type GenResult<T> = ::std::result::Result<T, Box<dyn Error>>;

// Dealing with errors that "can't" happen
// Call unwrap or expect on your Result value
