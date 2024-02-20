use std::io;

pub mod actions;

use crate::actions::*;

pub mod room;

pub mod game_data;
use game_data::Game;


fn main() {
    
    let mut game = Game {
        player_location: "room1",
    };

    println!("My dungeon crawler");

    let mut has_seen_description = false;
    
    loop {
        if has_seen_description == false {
            let _ = match_action("look", &mut game);
            has_seen_description = true;
        }

        let mut input = String::new();
        println!("What do you want to do?");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let result = match_action(&input, &mut game);
        match result {
            Ok(new_data) => {
                game = new_data;
            },
            Err(()) => {},
        }
    }

}
