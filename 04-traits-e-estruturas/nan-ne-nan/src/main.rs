fn main() {
    let a = 0.0f64 / 0.0f64;
    let b = 0.0f64 / 0.0f64;

    let c = 0.0f32 / 0.0f32;
    let d = 0.0f32 / 0.0f32;

    println!("a == b -> {}", a == b);
    println!("c == d -> {}\n", c == d);

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d)
}
