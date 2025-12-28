//! # r_pass
//! 
//! `r_pass` creates a secure* password string and prints it to the terminal for use. 
//! 
//! Future releases will contain an interactive mode as well as flags to provide for quick configuration of a password. 
//! 
//! # Usage
//! 
//! `./r_pass`

use rand::prelude::IndexedRandom;
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
    let keyspace = ['!','#','$','%','&','(',')','+','-','0','1','2','3','4','5','6','7','8','9',':',';','?','@','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','_','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    

    let mut rng = rand::rng();
    let mut password: String = String::new();
    let mut current_char: &char;
    for _ in 1..=16 {
        current_char = keyspace.choose(&mut rng).unwrap();
        password.push(*current_char);
    }

    println!("{password}");
}


/// Shows the usage prompt if required.
pub fn show_help() {
    println!("Usage: ./r_pass\n");
    println!("Creates a quick secure* secret key.");
    println!("*[16 character string, alphanumeric and symbols]")
}
