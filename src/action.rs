use crate::{Game, command::Command};

#[derive(Clone)]
pub struct Action<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub act: fn(&mut Game<'a>, Command),
}

impl<'a> Action<'a> {
    pub fn new(name: &'a str, description: &'a str, act: fn(&mut Game<'a>, Command)) -> Action<'a> {
        Action {
            name,
            description,
            act,
        }
    }
}
