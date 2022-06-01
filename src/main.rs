use std::io;
use std::process;

struct PhoneBook {
    name: Vec<String>,
    number: Vec<String>,
}

impl PhoneBook {
    fn list(&self) {
        println!("NAME \t\t CONTACT NUMBER");
        println!("------------------------");
        for i in 0..self.name.len() {
            println!("{} \t\t {}", self.name[i], self.number[i]);
        }
    }

    fn add(&mut self) {
        let mut user_name = String::new();
        println!("Enter Name: ");
        io::stdin().read_line(&mut user_name).expect("No username");
        let user_name: String = user_name
            .trim()
            .parse()
            .expect("Name entered was not a valid");
        self.name.push(user_name);

        let mut user_number = String::new();
        println!("Enter Number: ");
        io::stdin()
            .read_line(&mut user_number)
            .expect("No usernumber");
        let user_number: String = user_number
            .trim()
            .parse()
            .expect("Number entered was not a valid");
        self.number.push(user_number);
    }

    fn delete(&mut self) {
        let mut name_to_be_deleted = String::new();
        println!("Enter Name to be deleted: ");
        io::stdin()
            .read_line(&mut name_to_be_deleted)
            .expect("No username");
        let name_to_be_deleted: String = name_to_be_deleted
            .trim()
            .parse()
            .expect("Name entered was not a valid");

        for name_idx in 0..self.name.len() {
            if self.name[name_idx] == (name_to_be_deleted) {
                println!("found at index {}", name_idx);
                self.name.remove(name_idx);
                self.number.remove(name_idx);
            }
        }
    }

    fn find(&mut self) {
        let mut name_to_be_found = String::new();
        println!("Enter Name to be found: ");
        io::stdin()
            .read_line(&mut name_to_be_found)
            .expect("No username");
        let name_to_be_found: String = name_to_be_found
            .trim()
            .parse()
            .expect("Name entered was not a valid");

        for name_idx in 0..self.name.len() {
            if self.name[name_idx] == (name_to_be_found) {
                println!("Found {}", self.name[name_idx]);
                println!("\t Name:   {}", self.name[name_idx]);
                println!("\t Number: {}", self.number[name_idx]);
            }
        }
    }
}

fn main() {
    let mut phonebook_instance = PhoneBook {
        name: vec![],
        number: vec![],
    };

    loop {
        menu();

        let mut user_choice = String::new();

        io::stdin().read_line(&mut user_choice).expect("No input");

        let user_choice: u8 = user_choice
            .trim()
            .parse()
            .expect("user_choice entered was not a number");

        match user_choice {
            1 => phonebook_instance.list(),
            2 => phonebook_instance.add(),
            3 => phonebook_instance.delete(),
            4 => phonebook_instance.find(),
            5 => process::exit(1),
            _ => panic!("Wrong Choice"),
        }
    }
}

fn menu() {
    println!("*---------------------*");
    println!("My RUSTY Phonebook");
    println!("*---------------------*");
    println!("1. List   Contacts");
    println!("2. Add    Contacts");
    println!("3. Delete Contacts");
    println!("4. Find   Contacts");
    println!("5. EXIT");
    println!("*---------------------*");
    println!("Enter your choice: ");
}
