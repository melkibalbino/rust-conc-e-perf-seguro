fn main() {
    let a: f32 = 3.549236;
    println!("Floor: {}", a.floor());
    println!("Ceil: {}", a.ceil());
    println!("Round: {}", a.round());
    println!("Truncate: {}", a.trunc());
    println!("Fractional: {}", a.fract());

    println!("Is Finite?: {}", a.is_finite());
    println!("Is Infinite?: {}", a.is_infinite());
    println!("Is NaN?: {}", a.is_nan())
}
