use std::fmt;

#[derive(Debug)] // create an automatic implementation of Debug trait
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    // this is an example of function on a mutable User
    // it will not work on immutable User
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count = self.sign_in_count + 1
    }

    // this will return a new copy of a User, consuming self. Note: "self: Self" is a long for of a short form "self"
    // we could also replace "-> Self" with "-> User" => those do the same thing
    fn increment_sign_in_and_return(self: Self) -> Self {
        User {
            sign_in_count: self.sign_in_count + 1,
            ..self
        }
    }

    // this will return a value, but won't change it
    fn sign_in_plus_one(&self) -> u64 {
        self.sign_in_count + 1
    }
}

// implement Display for user to allow it to be used in formatted string
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User {{ active: {}, username: {} }}", self.active, self.username)
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // instead of saying username: username which is valid, we can just say this
        email,
        sign_in_count: 1,
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

// this is a method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // this is an Associated functions. You can all it with :: syntax
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// this is a unit-like struct
struct AlwaysEqual;

fn main() {
    // creating instances
    let usr = User {
        active: true,
        username: String::from("robot"),
        email: String::from("some@gmail.com"),
        sign_in_count: 32,
    };

    let usr_email = usr.email;

    println!("usr's email is '{usr_email}'");

    // Rust does not allow to mark only some values as mutable, only the whole struct could be mutable
    let mut changable_user = User {
        active: false,
        username: String::from("test"),
        email: String::from("other@gmail.com"),
        sign_in_count: 0,
    };

    println!("changable_user's active is '{}'", changable_user.active);
    changable_user.active = true;
    println!("changable_user's active is '{}'", changable_user.active);

    let user = build_user(String::from("email@mail.com"), String::from("username"));
    println!("user's name is '{}'", user.username);

    // There is also a short form to copy fields from existing struct only changing some of them
    // Note: this moves the user, hence the user can not be used afterward
    let user2 = User {
        sign_in_count: 47,
        email: String::from("another@example.com"),
        ..user
    };
    println!("user2's sign_in_count: '{}', email: '{}'", user2.sign_in_count, user2.email);

    // will fail on compile, because the '=' above moved user to user2
    //println!("norml user: {user}");

    // first line here works, because we have implemented Display for User struct
    // second line works, because we have derived a Debug for User struct
    println!("norml user2: {}", user2);
    println!("debug user2: {:?}", user2);

    // debug print prints the whole struct and also specify the line. In prints in system error
    dbg!(&user2);

    // we could also use dbg around expression, e.g. like this
    // notice how output looks like in a console
    let user3 = User {
        username: dbg!(String::from("another-username")),
        ..user2
    };
    println!("user3 debug {:?}", user3);

    // using methods

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is '{}' square pixels.",
        rect.area()
    );

    // next line won't compile, because user3 is immutable
    // user3.increment_sign_in_count()

    // but we can use a method to replace user3 in immutable fashion
    println!("user3.sign_in_count: '{}'", user3.sign_in_count);
    let user3 = user3.increment_sign_in_and_return();
    println!("user3.sign_in_count: '{}'", user3.sign_in_count);

    // We can also create a method that accepts a mutable reference
    let mut user4 = user3;
    println!("user4.sign_in_count: '{}'", user4.sign_in_count);
    user4.increment_sign_in_count();
    println!("user4.sign_in_count: '{}'", user4.sign_in_count);

    println!("sing_in_count + 1 is: '{}'", user4.sign_in_plus_one());
    println!("user4.sign_in_count: '{}'", user4.sign_in_count);

    let square = Rectangle::square(32);

    println!("square ares is height '{}' * width '{}' = '{}'", square.height, square.width, square.area())


}
