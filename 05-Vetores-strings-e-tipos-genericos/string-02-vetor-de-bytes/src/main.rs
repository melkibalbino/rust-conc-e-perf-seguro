fn main() {
    let vec = vec![82, 117, 115, 116];
    let a = String::from_utf8(vec).unwrap();
    println!("{}", a);
}
