#[derive(Debug)]
struct Contact {
    name: &'static str,
    phone_number: &'static str
}

fn main() {

    let contato_01 = Contact {
        name: "Sivirina Chique Chique",
        phone_number: "+55 (82) 93325-6554"
    };

    let contato_02 = Contact {
        name: "Maria Jose",
        phone_number: "+55 (81) 99877-9878"
    };

    let agenda = vec![contato_01, contato_02];

    let names = agenda.iter()
        .map(|contact| { contact.name })
        .collect::<Vec<_>>();

    println!("Names: {:?}", names);

    let phones = agenda.iter()
        .map(|contact| { contact.phone_number })
        .collect::<Vec<_>>();

    println!("Phones: {:?}", phones)

}
