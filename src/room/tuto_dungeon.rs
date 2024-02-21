use std::collections::HashMap;
use crate::room::*;


pub fn test_dungeon(id: &str) -> Room {
    let mut room1 = Room {
        description: "You're in a dark room. There is a sign hanging on the wall.",
        exits: HashMap::new(),
        items: vec![Items::Sign(vec!["sign"], "Welcome to my dungeon!")],
    };
    room1.exits.insert(Exits::North, "room2");

    let mut room2 = Room {
        description: "This is room number 2",
        exits: HashMap::new(),
        items: vec![Items::Sign(vec!["sign"], "This is the second sign.")],
    };
    room2.exits.insert(Exits::South, "room1");
    room2.exits.insert(Exits::North, "room3");

    let mut room3 = Room {
        description: "This is room number 3",
        exits: HashMap::new(),
        items: vec![Items::Sign(vec!["sign"], "This is the third sign.")],
    };
    room3.exits.insert(Exits::South, "room2");


    let mut dungeon = HashMap::new();
    dungeon.insert("room1", room1);
    dungeon.insert("room2", room2);

    return dungeon.get(id).cloned().unwrap();
}