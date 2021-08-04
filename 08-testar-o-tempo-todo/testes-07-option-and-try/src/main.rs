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
    let mut file = match File::create("my_best_friends.txt") {
        Err(e) => return Err(e),
        Ok(f) => f,
    };

    if let Err(e) = file.write_all(format!("name: {}\n", info.name).as_bytes()) {
        return Err(e);
    }

    if let Err(e) = file.write_all(format!("age: {}\n", info.age).as_bytes()) {
        return Err(e);
    }

    if let Err(e) = file.write_all(format!("rating: {}\n", info.rating).as_bytes()) {
        return Err(e);
    }
    Ok(())
}

fn main() {
    let friend01 = Info {
        name: "Nelson Castellani",
        age: 72,
        rating: 10,
    };

    match write_info(&friend01) {
        Err(e) => println!("Ops, something wrong -> {}", e),
        Ok(()) => println!("Everything is in the right place"),
    };
}
