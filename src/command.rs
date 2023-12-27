use std::io::{self, Write};

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn get(prompt: &str) -> Command {
        print!("{} ", String::from(prompt));
        // stdout needs to be flushed or else the prompt won't be displayed until
        //  after the command is entered
        drop(io::stdout().flush());

        let mut response: String = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Error reading command please try again.");

        let mut words = response.split_whitespace().map(|s: &str| s.to_owned());

        let command = Command {
            name: words.next().expect("Error parsing command."),
            args: words.collect(),
        };

        command
    }
}
