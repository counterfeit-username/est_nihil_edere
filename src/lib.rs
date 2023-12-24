mod action;
mod command;

use std::process::exit;

use action::Action;
use command::Command;

#[derive(Clone)]
pub struct Game<'a> {
    actions: Vec<Action<'a>>,
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game {
            actions: Vec::new(),
        }
    }
}

pub fn new_game<'a>() -> Game<'a> {
    let mut game = Game::new();
    game.actions = vec![
        Action::new(
            "help",
            "What do you think?",
            |game: &mut Game, _: Command| {
                println!("\nAll commands:");
                for action in &game.actions {
                    println!("{} - {}", action.name, action.description);
                }
            },
        ),
        Action::new("quit", "Exits the game.", |_: &mut Game, _: Command| {
            exit(0)
        }),
        Action::new(
            "parrot",
            "Repeats the first argument given",
            |_: &mut Game, command: Command| println!("\n{}", command.args[0]),
        ),
    ];

    game
}

pub fn run(mut game: Game) {
    loop {
        let command =
            Command::get("\nWhat would you like to do? (enter 'help' for a list of commands)\n>");

        let mut invalid = true;
        let command_name = command.name.to_owned();

        let game_copy = game.clone();
        for action in game_copy.actions {
            if command_name == action.name {
                (action.act)(&mut game, command);
                invalid = false;
                break;
            }
        }

        if invalid {
            println!("\n'{}' is not a valid action.", command_name);
        }
    }
}
