extern crate time;

fn print_today() {
    const THE_1900: i32 = 1900;

    let d = time::now();
    let day: i32 = d.tm_mday;
    let month: i32 = d.tm_mon;
    let year: i32 = d.tm_year + THE_1900;

    println!("Today is {}/{}/{}", day, month, year)
}

fn main() {
    print_today()
}
