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

}
