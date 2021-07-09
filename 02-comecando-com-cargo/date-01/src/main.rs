extern crate time;

fn main() {
    let d = time::now();

    println!("Todya is {}/{}/{}", d.tm_mday,
            d.tm_mon, d.tm_year + 1900)
}
