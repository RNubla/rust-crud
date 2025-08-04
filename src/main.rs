mod crud;
mod person;
use std::io::{self, Write};

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
    let id = user_crud.data.len() as i64 + 1;

    let first_name = prompt("Enter first name: ");
    let last_name = prompt("Enter last name: ");
    let age: i64 = loop {
        let input = prompt("Enter age: ");
        match input.trim().parse::<i64>() {
            Ok(n) => break n,
            Err(_) => println!("Invalid age. Please enter a number."),
        }
    };
    let person = Person {
        first_name: first_name.trim().to_string(),
        last_name: last_name.trim().to_string(),
        id,
        age,
    };

    user_crud.create(person);
    println!("User created successfully.");
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap(); // make sure the prompt shows
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
