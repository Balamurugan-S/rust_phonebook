use std::process;

use my_phonebook::PhoneBook;

fn main() {
    let mut phonebook_instance = PhoneBook {
        name: vec![],
        number: vec![],
    };

    loop {
        my_phonebook::menu();

        let user_choice = my_phonebook::get_user_choice();

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
