mod crud;
mod person;
use std::io;

use crud::{Crud, CrudTrait};
use person::{Person, PersonTrait};

fn main() {
    let mut user_crud: Crud<Person> = Crud { data: Vec::new() };
    loop {
        // Display menu
        println!();
        println!("Choose an option:");
        println!("  *  list users (l)");
        println!("  *  create user (c)");
        println!("  *  read user info (r)");
        println!("  *  update user (u)");
        println!("  *  delete user (d)");
        println!("  *  quit (q)");
        // print!("Your choice: ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "l" => println!("{:?}", user_crud.read_all_data()),
            "c" => {
                handle_create_user(&mut user_crud);
            }
            "r" => println!("read user"),
            "u" => println!("update user"),
            "d" => println!("delete user"),
            "q" => {
                println!("Quitting....");
                break;
            }
            _ => println!("Invalid Options. Please try again"),
        }
    }
}

fn handle_create_user(user_crud: &mut Crud<Person>) {
    loop {
        // println!(" Enter id");
        let id = user_crud.data.len() + 1;

        println!(" Enter first name");
        let mut f_name = String::new();
        io::stdin()
            .read_line(&mut f_name)
            .expect("Failed to read line");

        println!(" Enter last name");
        let mut l_name = String::new();
        io::stdin()
            .read_line(&mut l_name)
            .expect("Failed to read line");

        println!(" Enter age name");
        let mut age = String::new();
        io::stdin()
            .read_line(&mut age)
            .expect("Failed to read line");

        let person = Person {
            first_name: f_name.trim().to_string(),
            last_name: l_name.trim().to_string(),
            id: id as i64,
            age: age.trim().parse().expect("Age is not a string"),
        };
        user_crud.create(person);
        break;
    }
}
