fn main(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

let m = Message::Write(String::from("hello"));
m.call();
}

enum IpAddrKind {
    V4,
    V6
}

enum Message {
    Quit,
    Move{x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self) {

    }
}

enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter =>25,
    }
}