extern crate time;

fn main() {
    const THE_1900: i32 = 1900;

    let d = time::now();
    let day: i32 = d.tm_mday;
    let month: i32 = d.tm_mon;
    let year: i32 = d.tm_year + THE_1900;

    println!("Hoje é {}/{}/{}", day, month, year)
}
