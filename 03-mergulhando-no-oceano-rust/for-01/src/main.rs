fn main() {
    println!("Range Exclusivo");

    print!("|");
    for a in 1..10 { // for com range exclusivo
        print!(" {} |", a);
    }

    println!("\nRange Inclusivo");
    print!("|");
    for a in 1..=10 { 
        print!(" {} |", a)
    }
    println!()
}
