use std::collections::HashMap;

pub fn room_hash() {
    let mut hashmap : HashMap<&str, &str> = HashMap::new();
    hashmap.insert("room1", Room1);

}

static Room1 : &str = r#"
    {
        "id": "room1",
        "dungeon": "test_dungeon"&
        "description": "This is room number 1"
        "exits": [Exits::North]
    }
"#;
