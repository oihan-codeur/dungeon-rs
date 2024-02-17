pub fn match_action(guess: &str) {
    let guess = guess.trim();

    match guess {
        "look" => look(),
        "l" => look(),
        "north" => north(),
        "n" => north(),
        "south" => south(),
        "s" => south(),
        "east" => east(),
        "e" => east(),
        "west" => west(),
        "w" => west(),
        "quit" => quit(),
        "q" => quit(),
        _ => unknown(),
    };
}

pub fn look() {
    println!("You look around");
}

pub fn north() {
    println!("You go north");
}

pub fn south() {
    println!("You go south");
}

pub fn east() {
    println!("You go east");
}

pub fn west() {
    println!("You go west");
}

pub fn quit() {
    println!("Bye bye!");
    std::process::exit(0);
}

pub fn unknown() {
    println!("You can't do that");
}
