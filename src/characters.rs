use std::io;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub health: i32,
    pub alive: bool,
}

fn name_character() -> String {
    let mut name = String::from("");
    println!("Please name your character");
    io::stdin()
        .read_line(&mut name)
        .expect("Something went wrong, try again");

    name
}

pub fn start_new_player_character(game_on: bool) -> Character {
    let mut name = String::from("");
    let mut alive = false;

    while alive == false {
        println!("Seems as if you have no character created yet");
        println!("Avilible commands: [c]reate (character), [e]xit");

        let mut command = String::from("");
        io::stdin()
            .read_line(&mut command)
            .expect("not valid command, please try again");

        match command.trim().as_ref() {
            "e" | "exit" => game_on = false,
            "c" | "create" => {
                name = name_character();
                alive = true;
            }
            _ => (),
        }
    }

    let character = Character {
        name: name,
        health: 100,
        alive: alive,
    };

    character
}
