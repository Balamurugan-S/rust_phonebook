use std::io;

pub struct PhoneBook {
    pub name: Vec<String>,
    pub number: Vec<String>,
}

impl PhoneBook {
    pub fn list(&self) {
        println!("NAME \t\t CONTACT NUMBER");
        println!("------------------------");
        for i in 0..self.name.len() {
            println!("{} \t\t {}", self.name[i], self.number[i]);
        }
    }

    pub fn add(&mut self) {
        self.name.push(Self::get_user_name(&self));

        self.number.push(Self::get_user_number(&self));
    }

    pub fn delete(&mut self) {
        let name_to_be_deleted = Self::get_user_name(&self);

        if !Self::check_empty(&self) {
            for name_idx in 0..self.name.len() {
                if self.name[name_idx] == (name_to_be_deleted) {
                    println!("DELETED {}", self.name[name_idx]);
                    self.name.remove(name_idx);
                    self.number.remove(name_idx);
                }
            }
        }
    }

    pub fn find(&mut self) {
        let name_to_be_found = Self::get_user_name(&self);

        if !Self::check_empty(&self) {
            for name_idx in 0..self.name.len() {
                if self.name[name_idx] == (name_to_be_found) {
                    println!("Found {}", self.name[name_idx]);
                    println!("\t Name:   {}", self.name[name_idx]);
                    println!("\t Number: {}", self.number[name_idx]);
                }
            }
        }
    }

    fn get_user_name(&self) -> String {
        let mut user_name = String::new();
        println!("Enter Name: ");
        io::stdin().read_line(&mut user_name).expect("No username");
        let user_name: String = user_name
            .trim()
            .parse()
            .expect("Name entered was not a valid");
        user_name
    }

    fn get_user_number(&self) -> String {
        let mut user_number = String::new();
        println!("Enter Number: ");
        io::stdin()
            .read_line(&mut user_number)
            .expect("No usernumber");
        let user_number: String = user_number
            .trim()
            .parse()
            .expect("Number entered was not a valid");
        user_number
    }

    fn check_empty(&self) -> bool {
        if self.name.is_empty() {
            println!("No entries yet, Consider adding some info");
            return true;
        } else {
            return false;
        }
    }
}

pub fn menu() {
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

pub fn get_user_choice() -> u8 {
    let mut user_choice = String::new();

    io::stdin().read_line(&mut user_choice).expect("No input");

    let user_choice: u8 = user_choice
        .trim()
        .parse()
        .expect("user_choice entered was not a number");

    user_choice
}
