pub mod test_dungeon;

enum _Exits {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

pub struct Room {
    pub description: &'static str,
}
