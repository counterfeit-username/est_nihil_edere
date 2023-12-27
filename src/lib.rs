mod command;

use std::process::exit;

use command::Command;

pub fn run() {
    loop {
        let command =
            Command::get("\nWhat would you like to do? (enter 'help' for a list of commands)\n>");

        match command.name.as_str() {
            "help" => print_help(),
            "quit" => exit(0),
            _ => println!("'{}' is not a command.", command.name)
        }
    }
}

fn print_help() {
    let command_list = [
        "quit - Exits the game.",
        "help - What do you think?"
    ];

    for command in command_list {
        println!("{}", command);
    }
}