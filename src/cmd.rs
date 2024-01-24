use std::{error::Error, fmt, fs::File, process::Command};


#[derive(Debug, Clone, PartialEq)]
enum CommandError {
    Empty,
    StdoutEmpty,
    StdinEmpty,
    FileError(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::Empty => write!(f, "Error: Empty command"),
            CommandError::FileError(file_name) => write!(f, "Error: file not found: {}", file_name),
            CommandError::StdinEmpty => write!(f, "Error: No file given after <"),
            CommandError::StdoutEmpty => write!(f, "Error: No file given after >"),
        }
    }
}

impl Error for CommandError {}

trait ParseCommand{
    fn parse_args(&mut self, args: & Vec<&str>) -> Result<&mut Command, CommandError>;
}

/*
Iterate through args and look for file redirection
*/
impl ParseCommand for Command {
    fn parse_args(&mut self, args: &Vec<&str>) -> Result<&mut Command, CommandError> {
        let mut args_end = args.len();
        let mut iter = args.iter().enumerate().peekable();

        while let Some((index, &arg)) = iter.next() {
            if arg == ">" {
                // Check if there's a next item
                if let Some(&(_, file_name)) = iter.peek() {
                    let file = File::create(file_name).map_err(|_| CommandError::FileError(file_name.to_string()))?;
                    self.stdout(file);
                    args_end = args_end.min(index);
                } else {
                    return Err(CommandError::StdoutEmpty);
                }
            } else if arg == "<" {
                // Check if there's a next item
                if let Some(&(_, file_name)) = iter.peek() {
                    let file = File::open(file_name).map_err(|_| CommandError::FileError(file_name.to_string()))?;
                    self.stdin(file);
                    args_end = args_end.min(index);
                } else {
                    return Err(CommandError::StdinEmpty);
                }
            }
        }

        // end args at first < or >
        self.args(&args[..args_end]);

        Ok(self)
    }
}

// Executes cmd and args
fn exec_cmd(cmd: &mut Command){
    // Print the input
    println!("Executing command: {:?} {:?}", cmd.get_program(), cmd.get_args());

    match cmd.status() {
        Ok(status) =>{
            println!("Exit status {}", status);
        },
        Err(err) => {
            eprintln!("Error executing command: {}", err);
        }
    }
}

/* converts input string into process::Command */
fn parse_cmd(input: &str) -> Result<Command, CommandError>{
    // Trim beginning and end of whitespace
    let input = input.trim();

    // If input is empty just skip
    if input.is_empty() {
        return Err(CommandError::Empty);
    }

    // Split the string by whitespace
    let mut args: Vec<&str> = input.split_whitespace().collect();

    // cmd is first value, and args are the rest
    let cmd = args.remove(0);
    let mut cmd = Command::new(cmd);
    cmd.parse_args(&mut args)?;

    Ok(cmd)
}

// Parses input and executes it
pub fn cmd_handler(input: &str){
    match parse_cmd(input){
        Ok(mut cmd) => {
            exec_cmd(&mut cmd);
        },
        Err(err) => {
            eprintln!("Error parsing command: {}", err);
        }
    }
}

// --------------------------------------- Tests ---------------------------------------
#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, fs, io::{Read, Write}, path::Path};
    use crate::cmd::{cmd_handler, parse_cmd};
    use super::CommandError; 

    #[test]
    fn parse_cmd_test_args() {
        let input = String::from("     ls        -a -l\n");
        let cmd = parse_cmd(&input).unwrap();
        let args: Vec<&OsStr> = cmd.get_args().collect();
        assert_eq!(cmd.get_program(), "ls");
        assert_eq!(args, &["-a", "-l"]);
    }
    #[test]
    fn parse_cmd_test_no_args() {
        let input = "echo";
        let cmd = parse_cmd(input).unwrap();
        let args: Vec<&OsStr> = cmd.get_args().collect();
        assert_eq!(cmd.get_program(), "echo");
        assert!(args.is_empty());
    }
    #[test]
    #[should_panic(expected = "The function should panic")]
    fn parse_cmd_test_empty() {
        let input = String::from("\n");
        parse_cmd(&input).expect("The function should panic");
    }

    #[test]
    fn parse_cmd_test_redirect_stdout() {
        let stdout_file = Path::new("output.txt");
        let input = "echo hello > output.txt";
        cmd_handler(&input);

        // Read the contents of the file
        let mut file_content = String::new();
        let mut file = fs::File::open(stdout_file).expect("Failed to open file");
        file.read_to_string(&mut file_content).expect("Failed to read file");
        // Assert that the file contains the expected text "hello"
        assert_eq!(file_content.trim(), "hello");
    }

    fn create_input_file(input_file: &Path){
        // Open the file for writing
        let mut file = fs::File::create(input_file).unwrap();
        // Write "hello world" to the file
        file.write_all(b"hello world").unwrap();
    }

    // TODO: Add a test for parse_cmd_test_redirect_stdin
    // #[test]
    // fn parse_cmd_test_redirect_stdin() {
    //     let stdin_file = Path::new("input.txt");
    //     create_input_file(stdin_file);

    //     let input = "cat < input.txt";
    //     cmd_handler(&input);

    //      // Convert stdout bytes to a string
    //     let stdout_str = String::from_utf8_lossy(&output.stdout);

    //     // Check if the output contains "hello world"
    //     assert!(stdout_str.contains("hello world"));
    // }

    #[test]
    fn parse_cmd_test_redirect_both() {
        let stdin_file = Path::new("input.txt");
        let stdout_file = Path::new("output.txt");
        create_input_file(stdin_file);

        let input = "cat < input.txt > output.txt";
        cmd_handler(&input);

        // Read the contents of the file
        let mut file_content = String::new();
        let mut file = fs::File::open(stdout_file).expect("Failed to open file");
        file.read_to_string(&mut file_content).expect("Failed to read file");
        // Assert that the file contains the expected text "hello"
        assert_eq!(file_content.trim(), "hello world");
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: StdoutEmpty")]
    fn parse_cmd_test_stdout_missing_file() {
        let input = "echo hello >";
        parse_cmd(input).unwrap();
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: StdinEmpty")]
    fn parse_cmd_test_stdin_missing_file() {
        let input = "cat <";
        parse_cmd(input).unwrap();
    }

    #[test]
    fn parse_cmd_test_empty_command() {
        let input = "";
        match parse_cmd(input) {
            Ok(_) => panic!("The function should panic"),
            Err(err) => assert_eq!(err, CommandError::Empty),
        }
    }
}