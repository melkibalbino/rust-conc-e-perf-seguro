#[derive(Debug, Clone, Copy)]
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

    let contato_03 = Contact {
        name: "Jamaicano",
        phone_number: "+85 (56) 5885-8545"
    };

    let mut agenda = vec![contato_01, contato_02];

    println!("Agenda possue 2 itens:\n {:?}", agenda);

    agenda.push(contato_03);
    println!("Agenda agora possue 3 itens:\n {:?}", agenda);

    let _ = agenda.pop();
    println!("Agenda possue 2 itens novamente:\n {:?}", agenda);

    println!("{:?}", agenda[0]);
    println!("{:?}", agenda[1]);

    println!("Usando For:");

    for contact in agenda {
        println!("{:?}", contact);
    }

    println!("Usando iter() e next():");
    // For é uma forma sugar e seria equivalente ao proximo codigo
    let agenda2 = vec![contato_01, contato_02];
    let mut agenda_iter = agenda2.iter();
    println!("{:?}", agenda_iter.next().unwrap());
    println!("{:?}", agenda_iter.next().unwrap())
}
