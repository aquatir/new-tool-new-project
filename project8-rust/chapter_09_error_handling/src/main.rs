use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // this will panic a program, ending it
    // add RUST_BACKTRACE=full env var to get a much more verbose backtrace
    // panic!("crash and burn");

    // There is also a Result<T,E> which is returned from a lot of functions
    // where Ok<T> denotes a successful result and Err<E> an error
    let file_path = "hello.txt";
    match File::open(file_path) {
        Ok(_) => println!("File at path '{}' exists", file_path),
        Err(_) => println!("File at path '{}' doesn't exist", file_path),
    };

    let greeting_file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
