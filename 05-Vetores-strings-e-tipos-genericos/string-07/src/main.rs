fn main() {
    let mut name = String::from("Marcelo Castellani");
    let mut space = name.find(" ").unwrap_or(0);

    let first_name: String = name.drain(..space).collect();

    space = name.find(" ").unwrap_or(0);

    let last_name: String = name.drain(space..).collect();

    println!("First name: {}", first_name);
    println!("Last name: {}", last_name);

    println!("Original string is empty: {}", name.is_empty())
}
