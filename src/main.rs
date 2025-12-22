use crossterm::{event::{read, Event, KeyCode}, terminal};

mod lib;
mod types;

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    lib::io::program_welcome();
    
    //parse a yes or no response, else display exit message
    let response = lib::io::get_yes_no();

    if response == "y" {
        println!("Starting study timer...");
        lib::timer();
    } else {
        lib::io::exit_message();
    }

    Ok(())
}
