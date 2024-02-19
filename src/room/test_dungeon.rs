use std::collections::HashMap;
use crate::room::*;


pub fn test_dungeon(id: &str) -> Room {
    let mut room1 = Room {
        description: "This is room number 1",
        exits: HashMap::new(),
    };
    room1.exits.insert(Exits::North, "room2");

    let mut room2 = Room {
        description: "This is room number 2",
        exits: HashMap::new(),
    };
    room2.exits.insert(Exits::South, "room1");

    let mut dungeon = HashMap::new();
    dungeon.insert("room1", room1);
    dungeon.insert("room2", room2);

    return dungeon.get(id).cloned().unwrap();
}