use rust_study_timer::util;

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    util::io::program_welcome();
    
    //parse a yes or no response, else display exit message
    util::io::handle_yes_no(util::timer);

    Ok(())
}
