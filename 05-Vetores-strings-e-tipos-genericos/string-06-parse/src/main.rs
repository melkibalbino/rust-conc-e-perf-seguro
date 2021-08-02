fn main() {
    let a = String::from("8");
    let b = String::from("ERROR");

    let parse01 = match a.parse::<i8>() {
        Ok(c) =>c + 1,
        Err(_d) => 0,
    };
    println!("8 + 1 = {}", parse01);

    let parse02 = match b.parse::<i8>() {
        Ok(c) =>c + 1,
        Err(_d) => 0,
    };
    println!("8 + ERROR = {}", parse02)
}
