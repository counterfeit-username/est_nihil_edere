use crate::Game;

pub struct Action<'a> {
    pub name: &'a str,
    description: &'a str,
    act: fn(&mut Game<'static>),
}

impl<'a> Action <'a>{
    pub fn act(&self) {
        self.act();
    }
}