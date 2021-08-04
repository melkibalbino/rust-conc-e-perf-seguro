use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Info {
    name: &'static str,
    age: i32,
    rating: i32,
}

#[allow(deprecated)]
fn write_info(info: &Info) -> io::Result<()> {
    // Early return on error

    // try! esta depreciado, usar o operador ?
    // r# para permitir usar mesmo depreciado e #[allow(deprecated)] para não mostrar aviso
    let mut file = r#try!(File::create("my_best_friends.txt"));

    r#try!(file.write_all(format!("name: {}\n", info.name).as_bytes()));

    r#try!(file.write_all(format!("age: {}\n", info.age).as_bytes()));

    r#try!(file.write_all(format!("rating: {}\n", info.rating).as_bytes()));
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
