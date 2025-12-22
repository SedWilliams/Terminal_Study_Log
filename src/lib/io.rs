/*****************************************************
 * Program IO functions
 *****************************************************/

/*****************************************************
 * INPUT (handling) FUNCTIONS
 *****************************************************/

//blocking wait for yes/no input
pub fn get_yes_no() -> &str {
    unimplemented!();
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
    println!("");
    println!("Would you like to start a study timer? (y/n)...");
}

//exit msg, displays on program close
pub fn exit_message() {
    println!("Exiting the program. Goodbye!");
}
