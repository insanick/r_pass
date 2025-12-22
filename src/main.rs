use r_pass;



fn main() {
    let args = std::env::args();

    match args.len() {
        1 => r_pass::quick_build(),
        _ => {
            //Show help menu
            r_pass::show_help();
            }
            
    }
        
}
