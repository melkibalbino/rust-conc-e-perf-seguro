use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, Debug)]
struct Contact {
    name: &'static str,
    phone_number: &'static str,
}

impl Ord for Contact {
    fn cmp(&self, other: &Contact) -> Ordering {
        (self.name).cmp(&(other.name))
    }
}

fn main() {
    let c1 = Contact {
        name: "Zenaide",
        phone_number: "+55(11) 9.1234.5678",
    };
    let c2 = Contact {
        name: "Alzira",
        phone_number: "+55(11) 9.8765.4321",
    };
    let c3 = Contact {
        name: "Marcelo",
        phone_number: "+55(11) 9.9999.8888",
    };
    let c4 = Contact {
        name: "Ana",
        phone_number: "+55(11) 9.4567.7654",
    };

    let mut agenda = vec![c1, c2, c3, c4];

    println!("Before:\n {:?}", agenda);

    agenda.sort();

    println!("Before:\n {:?}", agenda);

    let numbers = vec![134, 12, 2, 45, 6];
    println!("{:?}", numbers.first());
    println!("{:?}", numbers.last());
}
