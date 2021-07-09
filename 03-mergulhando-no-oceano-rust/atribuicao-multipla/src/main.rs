extern crate time;

fn main() {
    let d = time::now();
    let (day, month, year) = (d.tm_mday,
        d.tm_mon, d.tm_year + 1900);

    println!("Todya is {}/{}/{}", day, month, year)
}
