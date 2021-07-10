use std::env;

fn main() {
    for argument in env::args() {

        println!("{}", argument)
    }

    let arguments: Vec<String> = env::args().collect();
    println!("{:?}", arguments);
    println!("{}", arguments[1]);
}
