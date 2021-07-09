extern crate time;

fn main() {
    let d = time::now();
    let day:   i32 = d.tm_mday;
    let month: i32 = d.tm_mon;
    let year:  i32 = d.tm_year + 1900;

    println!("Todya is {}/{}/{}", day, month, year)
}
