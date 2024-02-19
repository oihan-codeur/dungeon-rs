pub mod test_dungeon;
use std::collections::HashMap;

#[derive(Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
pub enum Exits {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub struct Room {
    pub description: &'static str,
    pub exits: HashMap<Exits, &'static str>,
}

impl Room {
    pub fn new(description: &'static str, exits: HashMap<Exits, &'static str>) -> Self {
        Self {
            description,
            exits,
        }
    }   
}