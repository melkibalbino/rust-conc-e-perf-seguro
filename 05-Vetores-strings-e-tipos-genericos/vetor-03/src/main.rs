#[derive(Debug)]
struct Contact {
    name: &'static str,
    phone_number: &'static str,
}

fn main() {
    let contact_1 = Contact {
        name: "Somebody",
        phone_number: "+55(11) 9.1234.5678"
    };

    let contact_2 = Contact {
        name: "Someone",
        phone_number: "+55(11) 9.8765.4321"
    };

    let agenda = vec![contact_1, contact_2];
    println!("{:?}", agenda);
}
