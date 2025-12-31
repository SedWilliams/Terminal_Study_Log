use crossterm::{event::{read, Event, KeyCode}, terminal};

/*****************************************************
 * Program IO functions
 *****************************************************/

/*****************************************************
 * INPUT (handling) FUNCTIONS
 *****************************************************/

//handle yes_no()
//
//Params:
//      callback function
//      yes or no string (pass ownership)
//
//if yes -> callback()
//if no -> terminate prgm

pub fn handle_yes_no(cb: fn()) {
    
    //wait for yes/no keypress
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    let result = loop {
        if let Event::Key(event) = read().expect("Failed to read event") {
            match event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    break String::from("y");
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    break String::from("n");
                }
                _ => {
                    continue;
                }
            }
        }
    };
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    
    //handle results based on user input
    if result == "y" {
        println!("Starting timer...");
        cb();
    } else {
        exit_message();
    }
}

//non-blocking wait for termination keypress
pub fn await_terminate() {
    unimplemented!();
}

/*****************************************************
 * OUTPUT FUNCTIONS
 *****************************************************/

//welcome message, displays on program start
pub fn program_welcome() {
    println!("--------------------------");
    println!("Terminal Study Timer...");
    println!("--------------------------");
        println!("Would you like to start a study timer? (y/n)...");
}

//exit msg, displays on program close
pub fn exit_message() {
    println!("Exiting the program. Goodbye!");
}
