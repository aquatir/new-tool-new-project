enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) -> String {
    // exclusive pattern matching. If you add new IP, e.g. V8, this code will no longer compile
    match ip_kind {
        IpAddrKind::V4 => { String::from("this is IP v4") }
        IpAddrKind::V6 => { String::from("this is IP v6") }
    }
}

// you can add data to your enums
enum IpAddrKindWithData {
    V4(String),
    V6(String),
}

// each variant in the enum may include other types and even other structures
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn execute_message(message: Message) -> String {
    match message {
        Message::Quit => { String::from("quitting") }
        Message::Move { x, y } => { format!("moving x: '{x}', y: '{y}'") }
        Message::Write( value) => { format!("printing {value}" ) }
        Message::ChangeColor(r, g, b) => { format!("Changing colors to r: '{r}', g: '{g}', b: '{b}'") }
    }
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: '{}'", route(four));
    println!("six: '{}'", route(six));

    // this also works
    println!("V4: '{}'", route(IpAddrKind::V4));
    println!("V6: '{}'", route(IpAddrKind::V6));

    let _home = IpAddrKindWithData::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKindWithData::V6(String::from("::1"));

    println!("{}", execute_message(Message::Quit));
    println!("{}", execute_message(Message::Move {x: 10, y: 15}));
    println!("{}", execute_message(Message::Write(String::from("text-text-text"))));
    println!("{}", execute_message(Message::ChangeColor(255, 255, 255)));

    // one of the build-in enum types is Option<T>
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_char: {:?}", some_char);
    println!("absent_number: {:?}", absent_number);
}
