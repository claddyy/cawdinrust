fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first.clone());
    println!("{full}, originally {first}"); // first is now used here
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}