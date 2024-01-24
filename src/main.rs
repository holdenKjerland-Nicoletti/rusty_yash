use std::io;
use std::io::Write;

mod cmd;

fn shell_loop() {
    // Infinite loop
    loop {
        // Print # prompt
        print!("# ");
        io::stdout().flush().unwrap(); // Ensure the prompt is immediately displayed

        // Create a mutable string to store user input
        let mut input = String::new();

        // Read a line from standard input
        match io::stdin().read_line(&mut input) {
            // Ctrl-D (EOF)
            Ok(0) =>{
                println!("Ctrl-D (EOF) pressed. Exiting.");
                break;
            }
            Ok(_) => {
                cmd::cmd_handler(&input);

                // Clear the string for the next iteration
                input.clear();
            },
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break; // Exit the loop on error
            }
        }
    }
}

fn main() {
    let _ = shell_loop();
}
