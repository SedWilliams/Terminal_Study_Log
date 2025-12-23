use rust_study_timer::util;

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    util::io::program_welcome();
    
    //parse a yes or no response, else display exit message
    let response = util::io::get_yes_no();

    if response == "y" {
        println!("Starting study timer...");
        util::timer();
    } else {
        util::io::exit_message();
    }

    Ok(())
}
