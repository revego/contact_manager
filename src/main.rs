#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
    email: String,
}

impl Contact {
    fn new(name: &str, phone: &str, email: &str) -> Contact {
        Contact {
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        }
    }
} 

use std::io::{self, Write};

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    loop {
	
    	println!("1. Aggiungi un contatto");
    	println!("2. Visualizza tutti i contatti");
    	println!("3. Ricerca contatto per nome");
    	println!("4. Cancella un contatto");
    	println!("5. Esci");
    	println!("Seleziona un'opzione");
        io::stdout().flush().unwrap();

	let mut choiche = String::new();
	io::stdin().read_line(&mut choiche).unwrap();
	let choiche: u32 = choiche.trim().parse().unwrap_or(0);

	match choiche {
		1 => add_contact(&mut contacts),
		2 => view_contacts(&contacts),
		3 => search_contact(&contacts),
		4 => delete_contact(&mut contacts),
		5 => break,
		_ => println!("Opzione non valida riprova"),

	}
    } 
}

fn add_contact(contacts: &mut Vec<Contact>){
	let (name, phone, email) = get_contact_details();
	let contact = Contact::new(&name, &phone, &email);
	contacts.push(contact);
	println!("Contatto aggiunto con successo");
}

fn get_contact_details() -> (String, String, String) {
	let mut name = String::new();
	let mut phone = String::new();
	let mut email = String::new();

	print!("Inserisci nome: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut name).unwrap();
	
	print!("Inserisci numero di telefono: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut phone).unwrap();

	print!("Inserisci una email: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut email).unwrap();

	(name.trim().to_string(), phone.trim().to_string(), email.trim().to_string())
}

fn view_contacts(contacts: &Vec<Contact>) {
	for (i, contact) in contacts.iter().enumerate() {
		println!("{} {:?}", i + 1, contact);
	}
	println!();
}

fn search_contact(contacts: &Vec<Contact>) {
	let mut name = String::new();
	println!("Inserisci il nome da vervare: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut name).unwrap();
	let name = name.trim();

	for contact in contacts.iter().filter(|c| c.name.contains(name)) {
		println!("{:?}", contact);
	}
	println!();
}

fn delete_contact(contacts: &mut Vec<Contact>) {
	let mut name = String::new();
	println!("Inserisci il nome del contatto da cancellare: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut name).unwrap();
	let name = name.trim();

	if let Some(pos) = contacts.iter().position(|c| c.name == name) {
		contacts.remove(pos);
		println!("Contatto cancellato con seuccesso! \n");
	} else {
		
		println!("Contatto non trovato!");
	}
}

