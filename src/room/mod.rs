pub mod tuto_dungeon;
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
#[derive(Eq, Hash, PartialEq)]
pub enum Items {
    Sign(Vec<&'static str>, &'static str),
    Key,
    Door,
    Potion,
    Dagger,
    None,
}

#[derive(Clone, Debug)]
pub struct Room {
    pub description: &'static str,
    pub exits: HashMap<Exits, &'static str>,
    pub items: Vec<Items>,
}

pub fn match_item(item: &str) -> Items {
    match item {
        "sign" => Items::Sign(vec![], ""),
        "key" => Items::Key,
        "door" => Items::Door,
        "potion" => Items::Potion,
        "dagger" => Items::Dagger,
        _ => Items::None,
    }
}