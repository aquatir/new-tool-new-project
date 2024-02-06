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

    // you can fix that by returning an ownership with functions
    let other_s = String::from("hey");
    let m = take_and_return_ownership(other_s);

    // wouldn't work, because other_s no longer valid
    // println!(other_s)
    // but m is valid now due to ownership
    print!("m value is {m}")
}

fn takes_ownership(some_string: String) {
    println!("dropping string '{}' after function invocation", some_string);
}

fn take_and_return_ownership(some_string: String) -> String {
    println!("Value of str '{}', returning ownership", some_string);
    some_string
}




