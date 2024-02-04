// Chapter 3 of The Rust Book — variables, mutability, data types, functions, control flow
fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    // does not compliance, because x is immutable
    // x = 6;
    println!("The value of x is: {x}");

    let mut y = 10;
    println!("The value of y is : {y}");
    y = 15;
    println!("The value of y is : {y}");
}
