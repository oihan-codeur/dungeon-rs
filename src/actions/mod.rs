use std::io::Cursor;

use crate::room::{test_dungeon::test_dungeon, Exits};
use crate::game_data::Game;

pub fn match_action(guess: &str, data: &mut Game) -> Result<Game, ()> {
    let guess = guess.trim();

    match guess {
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
        "quit" => Err(quit()),
        "q" => Err(quit()),
        _ => Err(unknown()),
    }
}

#[allow(unsafe_code)]
pub fn look(location: &str) -> () {
    let current_room = test_dungeon(location);

    println!("{}", current_room.description);
}

pub fn north(mut location: &'static str) -> Game {
    let current_room = test_dungeon(location);

    match current_room.exits.get(&Exits::North) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
    },
        None => { println!("You can't go north"); Game{ player_location: location } },
    }
}

pub fn south(mut location: &'static str) -> Game {
    let current_room = test_dungeon(location);

    match current_room.exits.get(&Exits::South) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
    },
        None => { println!("You can't go south"); Game{ player_location: location } },
    }
}

pub fn east(mut location: &'static str) -> Game {
    let current_room = test_dungeon(location);

    match current_room.exits.get(&Exits::East) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
    },
        None => { println!("You can't go north"); Game{ player_location: location } },
    }
}

pub fn west(mut location: &'static str) -> Game {
    let current_room = test_dungeon(location);

    match current_room.exits.get(&Exits::West) {
        Some(room) => {
            location = room;
            look(location);
            Game {
                player_location: location,
            }
    },
        None => { println!("You can't go north"); Game{ player_location: location } },
    }
}

pub fn quit() -> () {
    println!("Bye bye!");
    std::process::exit(0);
}

pub fn unknown() -> () {
    println!("You can't do that");
}
