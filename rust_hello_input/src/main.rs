// imports
use std::io;

#[allow(non_snake_case)]
fn main() {
    println!("Hello, world!");
    println!();
    mySurname();
}


#[allow(non_snake_case)]
fn mySurname () {
    let mut name = String::new();
    let mut Surname = String::new();

    // Takes the input and Stores it in a Variable
    println!("First Name? ");
    io::stdin().read_line(&mut name).expect("Something Went Wrong");
    println!();
    println!("Hello Mr.{}", name);


    // Does the Same Job Just matches it for Error & Doesen't Stores it in Some Variable.
    println!("Surname? ");
    match io::stdin().read_line(&mut Surname) {
        Ok(_) => {
            println!();
            println!("You Surname is {}", Surname)
        },
        Err(e) => {
            println!("Something Went Wrong! {}", e);
        }
    };
    println!("Bye");


}
