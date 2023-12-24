mod command;
mod action;

use action::Action;
use command::Command;

pub struct Game<'a> {
    actions: Vec<Action<'a>>
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game { actions: Vec::new() }
    }
}

pub fn new_game() -> Game<'static>{
    Game::new()
}

pub fn run(game: Game) {
    loop {
        let command = Command::get(
            "\nWhat would you like to do (enter 'help' for a list of commands)?",
        );

        for action in &game.actions {
            if command.name == action.name {
                action.act();
            }
        }
    }
}