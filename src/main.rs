use std::process;

use my_phonebook::PhoneBook;

fn main() {
    let mut phonebook_instance = PhoneBook::default();

    loop {
        my_phonebook::menu();

        match my_phonebook::get_user_choice() {
            Ok(user_input) => match user_input {
                1 => phonebook_instance.list(),
                2 => phonebook_instance.add(),
                3 => phonebook_instance.delete(),
                4 => phonebook_instance.find(),
                5 => process::exit(1),
                _ => println!("Wrong Choice, try again!"),
            },
            Err(error) => {
                println!("Error Generated: {}", error);
            }
        };
    }
}
