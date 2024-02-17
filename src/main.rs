use std::io;

pub mod actions;
use crate::actions::*;

pub mod room;

fn main() {
    println!("My dungeon crawler");
    
    loop {
        let mut input = String::new();
        println!("What do you want to do?");
        
        let room_id = "room1";

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match_action(&input);
    }
}


