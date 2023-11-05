//Tuple- Fixed size, created using csv in ()
use std::io;
fn tuples() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

fn array() {
    let a = [1,2,3,4,5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = a[index];
    println!("Value at index {index} is : {element}")
}