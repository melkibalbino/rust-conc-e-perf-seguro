fn main() {
    let a: i32 = 2048;
    {
    let b = a + 1;
    println!("{}", b);
    }
    // b esta em um escopo diferente
    // println!("{}", b);
}
