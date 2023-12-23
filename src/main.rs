use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    loop {
        let command =
            get_command("\nWhat would you like to do (enter 'help' for a list of commands)?");

        match command.name.as_str() {
            "echo" => println!("echo"),
            "quit" => exit(0),
            "help" => {
                println!("\n");
                for action in ACTION_DESCRIPTIONS {
                    println!("'{}' - {}", action.0, action.1);
                }
            }
            _ => println!("Command not recognized (enter 'help' to see a list of commands)"),
        }
    }
}

fn get_command(prompt: &str) -> Command {
    print!("{} ", String::from(prompt));
    // stdout needs to be flushed or else the prompt won't be displayed until
    //  after the command is entered
    drop(io::stdout().flush());

    let mut response: String = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Error reading command please try again.");

    let mut elements = response.split_whitespace().map(|s: &str| s.to_owned());

    let command = Command {
        name: elements.next().expect("Error parsing command."),
        args: elements.collect(),
    };

    command
}

struct Command {
    name: String,
    args: Vec<String>,
}

const ACTION_DESCRIPTIONS: [(&str, &str); 3] = [
    ("echo", "Prints 'echo' to the terminal."),
    ("quit", "Stops the game (without )."),
    ("help", "Take a guess, why don't you?."),
];
