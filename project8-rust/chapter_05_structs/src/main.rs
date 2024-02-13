struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // instead of saying username: username which is valid, we can just say this
        email,
        sign_in_count: 1,
    }
}

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
    print!("user's name is '{}'", user.username);
}
