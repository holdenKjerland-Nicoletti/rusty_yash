use std::process::Command;

fn exec_cmd(cmd: &str, args: Vec<&str>){
    // Print the input
    println!("Executing command: {}", cmd);

    match Command::new(cmd).args(args).status() {
        Ok(status) =>{
            println!("Exit status {}", status);
        },
        Err(err) => {
            eprintln!("Error executing command: {}", err);
        }
    }
}

fn parse_cmd(input: &String) -> (&str, Vec<&str>){
    // Trim beginning and end of whitespace
    let cmd = input.as_str().trim();

    // Split the string by whitespace
    let mut args: Vec<&str> = cmd.split_whitespace().collect();

    let cmd = args[0];
    args = args[1..].to_vec();

    return (cmd, args);
}

pub fn cmd_handler(input: &String){
    let (cmd, args) = parse_cmd(input);

    // Just skip empty command
    if cmd.len() == 0{
        return
    }

    // Execute command
    exec_cmd(cmd, args);
}

#[cfg(test)]
mod tests {
    use crate::cmd::parse_cmd;

    #[test]
    fn parse_cmd_test() {
        let input = String::from("     ls        -a -l\n");
        let (cmd, args) = parse_cmd(&input);
        assert_eq!(cmd, "ls");
        assert_eq!(args, ["-a", "-l"]);

        let input = String::from("       echo\n");
        let (cmd, args) = parse_cmd(&input);
        assert_eq!(cmd, "echo");
        let blank :Vec<&str> = [].to_vec();
        assert_eq!(args, blank);
    }
}