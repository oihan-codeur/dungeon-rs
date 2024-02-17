mod test_dungeon;

use test_dungeon::room_hash;

enum Exits {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

struct Room {
    id: Box<str>,
    dungeon: Box<str>,
    description: Box<str>,
    exits: Vec<Exits>,
}

pub fn get_room() {
    room_hash();
    
}
