use std::io;

fn main() {
    let greeting = "Hello, ";
    let mut name = String::new();

    io::stdin().read_line(&mut name);

    println!("{}{}", greeting, name)
}