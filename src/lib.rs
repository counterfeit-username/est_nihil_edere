mod command;
mod action;

use std::process::exit;

use action::Action;
use command::Command;

#[derive(Clone)]
pub struct Game<'a> {
    actions: Vec<Action<'a>>
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game { actions: Vec::new() }
    }
}

pub fn new_game<'a>() -> Game<'a>{
    let mut game = Game::new();
    game.actions = vec![
        Action::new("echo", "Prints 'echo' to the terminal.", |_: &mut Game| {println!("echo")}),
        Action::new("help", "What do you think?", |game: &mut Game| {
            for action in &game.actions {
                println!("{} - {}", action.name, action.description);
            }
        }),
        Action::new("quit", "Exits the game.", |_: &mut Game| {exit(0)})
    ];
    
    game
}

pub fn run(mut game: Game) {
    loop {
        let command = Command::get(
            "\nWhat would you like to do (enter 'help' for a list of commands)?",
        );

        let game_copy = game.clone();
        for action in game_copy.actions {
            if command.name == action.name {
                (action.act)(&mut game);
            }
        }
    }
}