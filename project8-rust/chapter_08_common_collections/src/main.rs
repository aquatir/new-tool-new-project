use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("The vector is '{:?}'", v);
    v.push(5);

    if let Some(element) = v.get(2) {
        println!("element under index 2 is '{}'", element)
    } else {
        println!("elements under index 2 does not exist")
    }

    // next piece will panic if value is not available, while .get(<..>) returns an Optional
    // let index_two_elements = &v[2];
    // println!("value of elements under index 2 is '{}'", index_two_elements);

    println!("The vector is '{:?}'", v);
    v.append(&mut vec![1, 2, 3]);
    println!("The vector is '{:?}'", v);

    if let Some(element) = v.get(2) {
        println!("element under index 2 is '{}'", element)
    } else {
        println!("elements under index 2 does not exist")
    }

    // you can iterate over the elements in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let str = "text".to_string();
    println!("value of str is '{:?}'", str);

    // push_str and push expands a string by either another string or by a character
    let mut s = String::from("foo");
    println!("value of s is '{:?}'", s);
    s.push_str("bar");
    println!("value of s is '{:?}'", s);
    s.push('!');
    println!("value of s is '{:?}'", s);

    // we can you format to simplify long string concatenations
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("value of s is '{:?}'", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("value of s is '{:?}'", s);

    // the next line will print a length as 24 because .len() in Rust count number of bytes, not number of characters
    // the cyrillic says "Hello".
    let hello = "Здравствуйте";
    println!("the len of '{}' is '{}' bytes", hello, hello.len());

    // this will work, because it slices the String on correct number of bytes to still make it representable
    // due to each character taking 2 bytes
    let s = &hello[0..4];
    println!("first few chars of hello are '{}'", s);

    // this will compile, but will fail in runtime!
    // let s = &hello[0..3];

    // both of those works, however, because each character takes only 1 byte
    let hello = "hello";
    let s = &hello[0..4];
    println!("first few chars of hello are '{}'", s);
    let s = &hello[0..3];
    println!("first few chars of hello are '{}'", s);

    // you can also iterate over characters with strings
    // you can also iterate over bytes with hello.bytes() if you need it
    let hello = "Здравствуйте";
    for c in hello.chars() {
        print!("{}-", c)
    }
    println!();

    // defining a hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("value of scores is '{:?}'", scores);

    // .get(..) on a hashmap returns an optional
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("value of score is '{:?}'", score);

    // we can iterate over both keys and values
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // you can update existing keys in a hashmap
    scores.insert(String::from("Blue"), 25);
    println!("value of scores is '{:?}'", scores);

    // entry => .or_insert will only change a key if it does not yet exist
    scores.entry(String::from("Red")).or_insert(100);
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(100);
    println!("value of scores is '{:?}'", scores);

    // another usa case for entry => .or_insert is to create a default value in the hashmap if it does not exist
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
