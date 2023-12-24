use crate::Game;

#[derive(Clone)]
pub struct Action<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub act: fn(&mut Game<'a>),
}

impl<'a> Action <'a>{
    pub fn new(name: &'a str, description: &'a str, act: fn(&mut Game<'a>)) -> Action<'a> {
        Action { name, description, act }
    }
}