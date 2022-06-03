use std::error::Error;
use std::io;

#[derive(Default)]
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
        match Self::get_user_name(&self) {
            Ok(user_name) => self.name.push(user_name),
            Err(error) => println!("{}", error),
        };

        match Self::get_user_number(&self) {
            Ok(user_number) => self.number.push(user_number),
            Err(error) => println!("{}", error),
        };
    }

    pub fn delete(&mut self) {
        match Self::get_user_name(&self) {
            Ok(name_to_be_deleted) => {
                for name_idx in 0..self.name.len() {
                    if self.name[name_idx] == (name_to_be_deleted) {
                        println!("DELETED {}", self.name[name_idx]);
                        self.name.remove(name_idx);
                        self.number.remove(name_idx);
                        break;
                    }
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        };
    }

    pub fn find(&mut self) {
        match Self::get_user_name(&self) {
            Ok(name_to_be_found) => {
                if !Self::check_empty(&self) {
                    for name_idx in 0..self.name.len() {
                        if self.name[name_idx] == (name_to_be_found) {
                            println!("Found {}", self.name[name_idx]);
                            println!("\t Name:   {}", self.name[name_idx]);
                            println!("\t Number: {}", self.number[name_idx]);
                            break;
                        }
                    }
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        };
    }

    fn get_user_name(&self) -> Result<String, Box<dyn Error>> {
        let mut user_name = String::new();
        println!("Enter Name: ");
        io::stdin().read_line(&mut user_name)?;
        let user_name = user_name.trim().to_string();
        if user_name.is_empty() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Enter valid input",
            )));
        }
        Ok(user_name)
    }

    fn get_user_number(&self) -> Result<String, Box<dyn Error>> {
        let mut user_number = String::new();
        println!("Enter Number: ");
        io::stdin().read_line(&mut user_number)?;
        let user_number = user_number.trim().to_string();
        if user_number.is_empty() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Enter valid input",
            )));
        }
        Ok(user_number)
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

pub fn get_user_choice() -> Result<u8, Box<dyn Error>> {
    let mut user_choice = String::new();

    io::stdin().read_line(&mut user_choice)?;

    let user_choice: u8 = user_choice.trim().parse()?;

    Ok(user_choice)
}
