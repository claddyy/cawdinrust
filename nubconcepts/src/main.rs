fn structex() {
    let user1 = User {
        email: String::from("madscientists1523@gmail.com"),
        username: String::from("claddy"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("mkgeek91@gmail.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
