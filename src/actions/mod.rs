use crate::game_data::Game;
use crate::room::*;
use crate::room::{academy::academy_dungeon, Exits};

pub fn match_action(guess: &str, data: &mut Game) -> Result<Game, ()> {
    let first_word = guess.trim().split_whitespace().next().unwrap_or("");
    let second_word = guess.trim().split_whitespace().nth(1).unwrap_or("");

    match first_word {
        "look" => Err(look(data.player_location)),
        "l" => Err(look(data.player_location)),
        "north" => Ok(north(data.player_location)),
        "n" => Ok(north(data.player_location)),
        "south" => Ok(south(data.player_location)),
        "s" => Ok(south(data.player_location)),
        "east" => Ok(east(data.player_location)),
        "e" => Ok(east(data.player_location)),
        "west" => Ok(west(data.player_location)),
        "w" => Ok(west(data.player_location)),
        "up" => Ok(up(data.player_location)),
        "u" => Ok(up(data.player_location)),
        "down" => Ok(down(data.player_location)),
        "d" => Ok(down(data.player_location)),
        "read" => Err(read(second_word, data.player_location)),
        "quit" => Err(quit()),
        "q" => Err(quit()),
        _ => Err(unknown()),
    }
}

pub fn look(location: &str) -> () {
    let current_room = academy_dungeon(location);

    println!("{}", current_room.description);
    println!("Exits: {:?}", current_room.exits.keys());
}

pub fn north(mut location: &'static str) -> Game {
    let current_room = academy_dungeon(location);

    match current_room.exits.get(&Exits::North) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
        }
        None => {
            println!("You can't go north.");
            Game {
                player_location: location,
            }
        }
    }
}

pub fn south(mut location: &'static str) -> Game {
    let current_room = academy_dungeon(location);

    match current_room.exits.get(&Exits::South) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
        }
        None => {
            println!("You can't go south.");
            Game {
                player_location: location,
            }
        }
    }
}

pub fn east(mut location: &'static str) -> Game {
    let current_room = academy_dungeon(location);

    match current_room.exits.get(&Exits::East) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
        }
        None => {
            println!("You can't go east.");
            Game {
                player_location: location,
            }
        }
    }
}

pub fn west(mut location: &'static str) -> Game {
    let current_room = academy_dungeon(location);

    match current_room.exits.get(&Exits::West) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
        }
        None => {
            println!("You can't go west.");
            Game {
                player_location: location,
            }
        }
    }
}

pub fn up(mut location: &'static str) -> Game {
    let current_room = academy_dungeon(location);

    match current_room.exits.get(&Exits::Up) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
        }
        None => {
            println!("You can't go up.");
            Game {
                player_location: location,
            }
        }
    }
}

pub fn down(mut location: &'static str) -> Game {
    let current_room = academy_dungeon(location);

    match current_room.exits.get(&Exits::Down) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
        }
        None => {
            println!("You can't go down.");
            Game {
                player_location: location,
            }
        }
    }
}

pub fn read(to_read: &str, location: &str) -> () {
    let room = academy_dungeon(location);

    for i in room.items {
        match i {
            Items::Sign(_vec, desc) => {
                if to_read == "sign" {
                    println!("{}", desc);
                } else {
                    println!("You can't read that.");
                }
            }
            _ => {}
        }
    }
}

pub fn quit() -> () {
    println!("Bye bye!");
    std::process::exit(0);
}

pub fn unknown() -> () {
    println!("You can't do that.");
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
