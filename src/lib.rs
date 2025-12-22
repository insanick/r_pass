//! # r_pass
//! 
//! `r_pass` creates a secure* password string and prints it to the terminal for use. 
//! 
//! Future releases will contain an interactive mode as well as flags to provide for quick configuration of a password. 
//! 
//! # Usage
//! 
//! `./r_pass`


use std::io::{self, Write};
use rand::Rng;
use sim_put::Input;
/// Quickly creates a random string of characters for use
/// 
/// # Example
/// 
/// ```
/// use r_pass;
/// 
/// fn main() {
///     r_pass::quick_build();
/// }
/// ```
/// 
/// # Panics
/// This function will panic with an except() used on io::stdout().flush()
/// 
/// This will be addresses gracefully in a future release 
/// 
pub fn quick_build() {
    let mut rng = rand::rng();
    let mut current_char: char;
    for _ in 1..=16 {
        current_char = rng.random_range('!'..='z');
        print!("{}", current_char);
    }
    print!("\n");
    io::stdout().flush().expect("Could not flush!");
}


/// Shows the usage prompt if required.
pub fn show_help() {
    println!("Usage: ./r_pass [options]\n");
    println!("\n[r_pass] will help create a password based on prompts or options[future-feature].");
    println!("When used without flags or prompts, creates a quick secure* secret key.");
    println!("*[16 character string, alphanumeric and symbols]")
}


#[derive(Debug)]
enum Complexity {
    Simple,
    Pin,
    Complex,
}

#[derive(Debug)]
pub struct Password {
    password: Vec<char>,
    length: u8,
    complexity: Complexity
}

impl Password {
    pub fn build() -> Password {
        Password {
            password: Vec::new(),
            length: Password::get_length(),
            complexity: Password::get_complexity()
        }
    }

    fn get_length() -> u8 {
        
        let new_length = Input::prompted_input("Password length");

        let new_length: u8 = new_length
            .trim()
            .parse()
            .expect("Not a number!");

        new_length
    }

    fn get_complexity() -> Complexity {
        Password::complexity_prompt();

        let choice = Input::prompted_input("Complexity");



        let choice: u8 = choice.trim().parse().expect("not a digit");

        match choice {
            1 => Complexity::Simple,
            2 => Complexity::Complex,
            3 => Complexity::Pin,
            _ => {
                println!("Not an option!");
                Password::get_complexity()
            }
        }
    }

    fn complexity_prompt() {
        println!("1) Simple (Chars and Nums)");
        println!("2) Complex (Adds Symbols)");
        println!("3) Pin Mode (Digits only)");
        }

}