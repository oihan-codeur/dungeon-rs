use crate::room::*;
use std::collections::HashMap;

pub fn academy_dungeon(id: &str) -> Room {
    let mut room1 = Room {
        description: "You're in a dark room. \nThere is a sign hanging on the wall.\nType 'read sign' to begin your adventure...",
        exits: HashMap::new(),
        items: vec![Items::Sign(
            vec!["sign"],
            "Welcome to the Academy dungeon!
Here, you will learn basic commands to start your journey in this game. You will also fight your first enemies.
The 'read' command you just used is a command you can use to read some items, wether they are in your inventory or in the room, as long as they are readable.
For example, you can read a book, but you can't read a key.
Now, type 'north' to go to the next room.",
        )],
    };
    room1.exits.insert(Exits::North, "room2");

    let mut room2 = Room {
        description: "A torch is burning in this room. Your eyes hurt for a second, but you get used quickly. There is a sign on the wall.",
        exits: HashMap::new(),
        items: vec![
            Items::Sign(
                vec!["sign"],
                "The 'north' command make you move north. Most commands are pretty straight forward.
You can also move 'south', 'east' and 'west'. You can also go 'up' and 'down'.
However, you're not obligated to type the complete command, you can also type 'n' for north, 's' for south, and so on.
These are called shortcuts, and there are shortcuts for the most used commands.
For example, the 'look' command that you can use to see again the description printed when you enter a room has a shortcut, 'l'.
When you enter a room or type the 'look' command, you can see the available exits at the end of the description.
However, pay attention to the rest of the description, because some exits may exist, but are locked. In this case, they aren't printed.
Now, type 'look' or its shortcut and based on the output, determine what command you have to use to go to the next room."
            )],
    };
    room2.exits.insert(Exits::South, "room1");
    room2.exits.insert(Exits::East, "room3");

    let mut room3 = Room {
        description: "This is the basic item room",
        exits: HashMap::new(),
        items: vec![Items::Sign(vec!["sign"], "This is the third sign.")],
    };
    room3.exits.insert(Exits::West, "room2");
    room3.exits.insert(Exits::Up, "room4");

    let mut room4 = Room {
        description: "This is the door room",
        exits: HashMap::new(),
        items: vec![Items::Sign(vec!["sign"], "This is the fourth sign.")],
    };
    room4.exits.insert(Exits::Down, "room3");

    let mut room5 = Room {
        description: "This is the basic inventory room",
        exits: HashMap::new(),
        items: vec![],
    };

    let mut room6 = Room {
        description: "This is the basic fighting room",
        exits: HashMap::new(),
        items: vec![],
    };

    let mut dungeon = HashMap::new();
    dungeon.insert("room1", room1);
    dungeon.insert("room2", room2);
    dungeon.insert("room3", room3);
    dungeon.insert("room4", room4);
    dungeon.insert("room5", room5);
    dungeon.insert("room6", room6);

    dungeon.get(id).cloned().unwrap()
}
