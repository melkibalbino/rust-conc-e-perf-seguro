fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let b = &a[..];
    let c = &a[3..5];

    println!("a possui {} elementos.", a.len());
    println!("b possui {} elementos.", b.len());
    println!("c possui {} elementos.", c.len());
}
