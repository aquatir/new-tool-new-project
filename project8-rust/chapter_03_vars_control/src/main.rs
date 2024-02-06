// Chapter 3 of The Rust Book — variables, mutability, data types, functions, control flow

use rand::Rng;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constants mush always have a type

fn main() {
    //
    // variables, chars, numbers, arrays
    //

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
    println!(
        "type value 0: '{}', type value 1 '{}' type value 2 '{}'",
        a.0, a.1, a.2
    );

    // Rust also have arrays
    // you specify the size of the array in type. Rust will figure out a type if you don't do it
    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // knowing the type of the array, Rust knows that certain operations are invalid
    // println!("the 12th month is: {}", _months[12]) // Doesn't compile

    //
    // Functions
    //

    print_hello_world_function();
    let hey = "hey";
    let capitalized_hey = capitalize_function(hey);
    println!("hey: '{hey}', capitalized_hey: '{capitalized_hey}'");

    // can also return more than one value
    let (result, result_x_2) = return_val_and_double(5);
    println!("result: '{result}', result_x_2: '{result_x_2}'");

    //
    // If Statement
    //

    println!("spinning the coin...");
    let coin = rand::thread_rng().gen_bool(0.5);
    if coin {
        println!("It's tails")
    } else {
        println!("It's heads")
    }

    let one_to_four = rand::thread_rng().gen_range(1..=4);
    if one_to_four == 1 {
        println!("got a one")
    } else if one_to_four == 2 {
        println!("got a two")
    } else if one_to_four == 3 {
        println!("got a three")
    } else {
        println!("got a four")
    }

    // can use if as expression
    let coin = if rand::thread_rng().gen_bool(0.5) {
        "tails"
    } else {
        "heads"
    };
    println!("The result of another coin spin is {}", coin);

    //
    // Loops
    //

    // loop == infinite loop
    // loops can be expressions
    let mut times = 5;
    let stopped_after = loop {
        println!("again!");
        times = times - 1;
        if times == 0 {
            break times;
        }
    };
    println!("stopped the look after '{}' iterations", 5 - stopped_after);

    // loops can have labels to check where to break
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut i = 3;
    while i > 0 {
        println!("decrementing i from '{i}'");
        i = i - 1;
    }

    // looping on collections
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("cur element: '{element}'")
    }

    // can loop over ranges and other way around as well
    // 1..4 => 1,2,3
    // 1..=4 => 1,2,3,4
    for element in (1..4).rev() {
        println!("cur element '{element}'")
    }

    for element in 6..=10 {
        println!("fib({element}) is {}", fib(element));
    }
}

fn print_hello_world_function() {
    println!("Hello, world")
}

fn capitalize_function(str: &str) -> String {
    str.to_uppercase()
}

fn fib(n: u32) -> u32 {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        fib(n-2) + fib (n-1)
    }
}

fn return_val_and_double(x: u32) -> (u32, u32) {
    (x, x * 2)
}
