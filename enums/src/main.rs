enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6,
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}
enum Message {
    Quit,
    _Move { x: i32, y: i32 }, //structs inside enums to group them
    _Write(String),           //tuple structs
    _ChangeColor(i32, i32, i32),
}
impl Message {
    // can be implemented
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}
#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}
fn main() {
    let _four = IpAddressKind::V4;
    let _six = IpAddressKind::V6;
    let _localhost = IpAddressKind::V4(127, 0, 0, 1);
    let _msg = Message::Quit;
    let new_ip = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("ip"),
    };
    println!("IP: {} ", new_ip.address);

    // option enum
    let _some_no = Some(5);
    let _some_text = Some("String");
    let null_value: Option<i32> = None;
    // we cant add option type to other data even same option data type
    let a = 10;
    // let sum = a + null_value;
    let _sum = a + null_value.unwrap_or(1); // set default value to option enum

    // Adding one to option var
    let five: Option<i32> = Some(5);
    let six = plus_one(five);
    match six {
        Some(6) => println!("six"),
        _ => (),
    };
    if let Some(6) = six {
        println!("Number: Six");
    }

    // Pattern Matching
    value_in_cents(Coin::Quarter(State::Alabama));
}
fn _route(_ip_kind: IpAddressKind) {}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 2,
        Coin::Nickel => 3,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("State minted: {:#?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, //none return none
        Some(i) => Some(i + 1), // value returns value + 1
                       // _ => (), any other value
    }
}
