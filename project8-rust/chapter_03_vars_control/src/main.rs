// Chapter 3 of The Rust Book — variables, mutability, data types, functions, control flow

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constants mush always have a type

fn main() {
    let x = 5;
    println!("The value of x is: '{x}'");

    // does not compliance, because x is immutable
    // x = 6;
    // println!("The value of x is: {x}");

    let mut y = 10;
    println!("The value of y is : '{y}'");
    y = 15;
    println!("The value of y is : '{y}'");

    println!("There are '{THREE_HOURS_IN_SECONDS}' seconds in one hour");

    // shadowing: this will first print 12, due to inner block, then print 6 because we have exited the inner block
    // also the first 'let x' shadows the let x above
    // shadowing can even shadow immutable var with a mutable one
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: '{x}'");
    }
    println!("The value of x is: '{x}'");

    // shadowing effectively creates a new variable, so you can "change type"
    let x = "hello";
    println!("The value of x is: '{x}'");

    // Rust support different notations for Decimal,  Hex, Octal, Binary, Byte representations
    let _x = 98_923;
    let _x = 0xff;
    let _x = 0o77;
    let _x = 0b1111_0000;
    let _x = b'A';

    let x: u8 = 250;
    let (x, is_overflow) = x.overflowing_add(50);
    println!("Did 250 + 50 u8 overflow? {is_overflow}. Value of x is {x} or 250 + 50 - 256");

    // Rust characters are unicode scalar characters
    let _c = 'z';
    let _c: char = 'ℤ'; // with explicit type annotation
    let _c = '😻';

    // Rust also have types.
    // In the example below, you could drop the explicit types, but then a would become i32 as it's a default, not u32
    let (a, b, c): (u32, &str, f64) = (5, "test", 34.23);
    println!("type values are a: '{a}', b: '{b}', c: '{c}'");
    // Destructing is also supported
    let a = (5, 32, 43);
    println!("type value 0: '{}', type value 1 '{}' type value 2 '{}'", a.0, a.1, a.2);

    // Rust also have arrays
    // you specify the size of the array in type. Rust will figure out a type if you don't do it
    let _months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // knowing the type of the array, Rust knows that certain operations are invalid
    // println!("the 12th month is: {}", _months[12]) // Doesn't compile

}
