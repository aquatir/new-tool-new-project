use std::error::Error;
use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};

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

    // next block either return a file handler if file already exists, or creates a new file
    let _greeting_file = match File::open(file_path) {
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

    // A more advanced way to write it with Result.unwrap_or_else and closures to avoid nested match clauses
    let _greeting_file = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // If we don't need an action when a file does not exist, we might as well fail the whole program
    // with a panic by using .expect to pass the errors message, or simply .unwrap() to now pass it
    let _greeting_file = File::open(file_path)
        .expect(&format!("{} should be included in this project", file_path));

    match read_username_from_file() {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }

    match read_username_from_file_short() {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }

    match read_username_from_file_even_shorter() {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }

}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


// the more advanced way to write this function is this using '?' operator
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}