fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("s value is '{s}'");

    let s1 = String::from("hello");

    // here a "move" occurs, because Rust will copy the points, length and capacity from s1 to s2, making s1 no longer usable
    let s2 = s1;

    // now all the important String data has moved to s2, so the next commented line will be a compile-time error
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // you may also copy the stack with an explicit .clone() method call
    let s3 = String::from("goodbye");
    let s4 = s3.clone();
    print!("s3: '{s3}', s4: '{s4}'");

    let s = String::from("hello");
    takes_ownership(s);

    // no longer works, because the previous functions has dropped s already
    // takes_ownership(s);

    // you can fix that by returning an ownership with functions thanks to "shadowing"
    let other_s = String::from("hey");
    let m = take_and_return_ownership(other_s);

    // wouldn't work, because other_s no longer valid
    // println!(other_s)
    // but m is valid now due to ownership
    println!("m value is {m}");

    // the perhaps better way is to pass b -reference instead of pass by value
    // this way the ownership doesn't change, because you are simply passing in a reference, not the actual value
    // the act of creating a reference is called "borrowing" in Rust
    let s = String::from("hello");
    let len = str_length(&s);
    println!("String '{s}' has a length of '{len}'");

    // the references are immutable by default, but you can explicitly pass them as mutable, but in order to do it
    // the value should be mutable to begin with
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("String after append_world is '{s}'");

    // you can not have more than one mutable reference at a time. The following 3 lines will fail
    // let r1 = &mut s;
    // let r2 = &mut s; // this line will fail with "second mutable borrow occurs here
    // println!("{}, {}", r1, r2);

    // To have more than once reference, they should all be in a separate scope
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 value: '{r1}'")
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("r2 value: '{r2}'");

    // Rust does not allow dangling references

    //
    // Slices
    //

    // problem with this code is that the first_word function's return type is completely separate from
    // a string, so you can first call this function, then s.clear() the string, and the code will still work
    // but now the first_word_last_index will have a value that can not be used in the program.
    // the slices example later on allows us to prevent this bug
    let mut s = String::from("hello world");
    let first_word_last_index = first_word(&s);
    println!("first word in '{s}' ends on index {first_word_last_index}");
    if let Some(char_at_last_word) = s.chars().nth(first_word_last_index) {
        println!("Character at position {first_word_last_index} is '{}'", char_at_last_word);
    } else {
        println!("String is too short to get character at position {first_word_last_index}.");
    }
    s.clear();
    println!("s is now '{s}', but first_word_last_index still shows '{first_word_last_index}'");


    let s = String::from("hello world");

    let hello = &s[0..5]; // you can also write &s[..5]
    let world = &s[6..11]; // can also write %s[6..len]
    println!("Two strings are '{hello}'_'{world}'");

    // Using slice in a function
    // the function will create a slice which is a reference to s
    // meaning that later on you can no longer remove s because there is a reference to it
    let mut s = &String::from("hello word");
    let res = first_word_slice(s);
    println!("s is '{s}', res is '{res}'");

    // this does not compile
    // s.clear();
}

fn append_world(p0: &mut String) {
    p0.push_str(", world")
}

fn str_length(str: &String) -> usize {
    // this will not work, because we don't own str, we simply borrowed it
    // str.push_str("kek");

    str.len()
}

fn takes_ownership(some_string: String) {
    println!("dropping string '{}' after function invocation", some_string);
}

fn take_and_return_ownership(some_string: String) -> String {
    println!("Value of str '{}', returning ownership", some_string);
    some_string
}

/// Return an index of the end of the first word in a String
/// the index will either point to the space right after the first word or point right after the end of the string
/// if there are no words
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// a better way to design this API is to make s: &str too, because it will work with both slices and Strings
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}




