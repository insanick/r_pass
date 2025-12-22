use std::io::{self, Write};
use rand::Rng;

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
    println!("Usage: ./r_pass\n");
    println!("Creates a quick secure* secret key.");
    println!("*[16 character string, alphanumeric and symbols]")
}
