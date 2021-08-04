use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Info {
    name: &'static str,
    age: i32,
    rating: i32,
}

fn write_info(info: &Info) -> io::Result<()> {
    // Early return on error

    let mut file = File::create("my_best_friends.txt")?;

    file.write_all(format!("name: {}\n", info.name).as_bytes())?;

    file.write_all(format!("age: {}\n", info.age).as_bytes())?;

    file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;

    Ok(())
}

fn main() {
    let friend01 = Info {
        name: "João Otario",
        age: 31,
        rating: 0,
    };

    match write_info(&friend01) {
        Err(e) => println!("Ops, something wrong -> {}", e),
        Ok(()) => println!("Everything is in the right place"),
    };
}
