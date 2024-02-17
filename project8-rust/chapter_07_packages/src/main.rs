use crate::garden::vegetables::Asparagus;

// Tells the compiler to include the code in src/garden.rs
pub mod garden;
mod libs;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    // use can go on top or be inlined
    use libs::front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();

    libs::cook_meal();
}
