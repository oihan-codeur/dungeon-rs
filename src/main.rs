use std::io;

pub mod actions;
use crate::actions::*;

pub mod room;

fn main() {
    println!("My dungeon crawler");

    let mut has_seen_description = false;
    
    loop {
        if has_seen_description == false {
            match_action("look");
            has_seen_description = true;
        }

        let mut input = String::new();
        println!("What do you want to do?");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match_action(&input);
    }
}