use crossterm::{event::{read, Event, KeyCode}, terminal};

mod lib;
mod types;

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    lib::io::program_welcome();

    //match input to handle yes/no response for starting the timer
    loop {

        terminal::enable_raw_mode().expect("Failed to enable raw mode");

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    println!("Starting study timer...");
                    lib::timer();
                    break;
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    // println!("Exiting the program. Goodbye!");
                    lib::io::exit_message();
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    } 


    terminal::disable_raw_mode().expect("Failed to disable raw mode");

    Ok(())
}
