enum IpTpye {
    V4(u32, u32, u32, u32),
    V6(u32, u32, u32, u32, u32, u32),
}

struct IpAddress {
    ip_type: IpTpye,
    addr: String,
}

enum Message {
    Quite,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn do_something() {
        println!("Silly Function");
    }
}

#[derive(Debug)]
enum UsState {
    California,
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn main() {
    let v4 = IpTpye::V4;
    let v6 = IpTpye::V6;

    // let addr = IpAddress{ip_type: IpTpye::V4, addr: String::from("127.0.0.1")};
    let add = IpTpye::V4(127, 0, 0, 1);

    // Option Enum

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some(String::from("something"));

    let none: Option<i32> = None;

    let x = 10;
    let y = Some(2);
    // let y: Option<i32> = None;

    // let sum = x + y;
    let sum = x + y.unwrap_or(0);
    println!("SUM: {}", sum);

    println!("Cents: {}", value_in_cents(Coin::Dime));
    println!("Cents: {}", value_in_cents(Coin::Penny));
    println!("Cents: {}", value_in_cents(Coin::Quater(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);

    println!("Six: {}", six.unwrap_or(0));
    let none = plus_one(None);

    print_val_if_present(Some(3));
    print_val_if_present(None);
}

fn route(v: IpTpye) {}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("Quater is from {:#?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,
        Some(i) => Some(i + 1),
        _ => None,
    }
}

fn print_val_if_present(x: Option<i32>) {
    match x {
        Some(i) => println!("Value Present {}", i),
        _ => (),
    }
}
