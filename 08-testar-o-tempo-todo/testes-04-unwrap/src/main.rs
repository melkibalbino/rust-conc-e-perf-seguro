fn main() {
    let str: &'static str = "String is not a number";
    let num: i32 = str.parse().unwrap();
    println!("{}", num)
}
