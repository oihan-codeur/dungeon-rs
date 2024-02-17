use std::collections::HashMap;
use crate::room::*;

pub fn get_room(id: &str) -> &Room {
    let mut hashmap : HashMap<&str, &Room> = HashMap::new();
    hashmap.insert("room1", &ROOM_ONE);

    let current_room = hashmap.get(id).unwrap();

    current_room
}

static DESCRIPTION: &'static str = "This is room number 1";

static ROOM_ONE: Room = Room {
    description: DESCRIPTION,
};