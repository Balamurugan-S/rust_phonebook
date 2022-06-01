use std::io;
use std::process;

struct PhoneBook {
    name: String,
    number: String,
}

impl PhoneBook {
    fn list(&self) {
        println!("{} {}", self.name, self.number);
    }

    fn add(&self) {
        println!("Adding data");
        Self::get_user_name();
        Self::get_user_number();
    }

    fn get_user_name() {
        let mut user_name = String::new();
        println!("Enter Name: ");
        io::stdin().read_line(&mut user_name).expect("No username");
        let user_name: String = user_name
            .trim()
            .parse()
            .expect("Name entered was not a valid");
        self.name = user_name;
    }

    fn get_user_number() {
        let mut user_number = String::new();
        println!("Enter Number: ");
        io::stdin()
            .read_line(&mut user_number)
            .expect("No usernumber");
        let user_number: String = user_number
            .trim()
            .parse()
            .expect("Number entered was not a valid");
        self.number = user_number;
    }
}

fn main() {
    let phonebook_instance = PhoneBook {
        name: String::from("John"),
        number: String::from("1234567890"),
    };

    loop {
        menu();

        let mut user_choice = String::new();

        io::stdin().read_line(&mut user_choice).expect("No input");

        let user_choice: u8 = user_choice
            .trim()
            .parse()
            .expect("user_choice entered was not a number");

        println!("You have entered: {}", user_choice);

        match user_choice {
            1 => phonebook_instance.list(),
            2 => phonebook_instance.add(),
            5 => process::exit(1),
            _ => panic!("Wrong Choice"),
        }
    }
}

fn menu() {
    println!("My RUSTY Phonebook");
    println!("1. List   Contacts");
    println!("2. Add    Contacts");
    println!("3. Delete Contacts");
    println!("4. Find   Contacts");
    println!("5. EXIT");
    println!("Enter your choice: ");
}
