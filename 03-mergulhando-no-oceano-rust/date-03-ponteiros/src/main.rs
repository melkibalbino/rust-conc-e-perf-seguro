extern crate time;

fn print_today() {
    const THE_1900: i32 = 1900;

    let d = time::now();

    let day:   i32 = d.tm_mday;
    let month: i32 = d.tm_mon;
    let year:  i32 = d.tm_year + THE_1900;

    println!("Hoje é {}/{}/{}", day, month, year)
}

fn do_the_things(function: fn()){
    function()
}

fn main() {
    // let pointer_to_function: fn() = print_today;
    let pointer_to_function = print_today; // Com inferencia de tipo

    // Passando print_today por referencia
    do_the_things(pointer_to_function)
}
