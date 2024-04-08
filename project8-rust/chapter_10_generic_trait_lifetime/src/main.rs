use std::fmt::Display;

use chapter_10_generic_trait_lifetime::{NewsArticle, Summary, Tweet};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Vector {:?}. The largest number is {}", number_list, result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("Vector {:?}. The largest number is {}", number_list, result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("Vector {:?}. The largest char is {}", char_list, result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?}", integer);
    println!("{:?}", float);


    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    println!("Point 1 {:?}", p1);
    println!("Point 2 {:?}", p2);
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let news = NewsArticle {
        headline: "headline".to_string(),
        location: "location".to_string(),
        author: "author".to_string(),
        content: "context".to_string(),
    };

    let tweet = Tweet {
        username: "username".to_string(),
        content: "context".to_string(),
        reply: false,
        retweet: false,
    };

    let some_struct = SomeStruct;

    println!("news: {}", news.summarize());
    println!("tweet: {}", tweet.summarize());
    println!("some_struct: {}", some_struct.summarize());

    notify(&tweet);
    notify(&news);


    // won't compile, because the lifetime of x is shorter than of r
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("part: {:?}, whole: {:?}", i.part, i);

    println!("level {:?}", i.level());

    let s: &'static str = "I have a static lifetime.";
    println!("s: {:?}", s);
}

// lifetime elision rules:
// 1. apply a different lifetime parameter to each input reference
// 2. if only 1 input, apply the same lifetime parameter to output
// 3. if class with &self, apply &self as a lifetime to output
// If everything fails, a user must specify a lifetime.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// using lifetime + generic parameter, where a generic type must implement Display
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// Struct with 2 generic parameters
// Rust perform monomorphization during complication, meaning that there will be multiple "copies"
// of this generic definition for each particular use case found in a code base
#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

struct SomeStruct;

// "No" implementation => uses a default implementation
impl Summary for SomeStruct {
    fn summarize_author(&self) -> String {
        String::from("no author")
    }
}

// Function accepting anything that implements a Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Above is a synthetic sugar over a trait bound form
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// you can also require more than one trait with a + sign
// pub fn notify(item: &(impl Summary + Display)) {
// OR
// pub fn notify<T: Summary + Display>(item: &T) {

// there is also a short for multi-trait argument, e.g. instead of this
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// you can write
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


// Generic parameter that implement PartialOrd trait
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
