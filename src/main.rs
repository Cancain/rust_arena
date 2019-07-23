mod characters;
mod fight;

use characters::Character;

struct Player {
    character: Character,
    has_character: bool,
    player_name: String,
}

fn main() {
    let dummy_character = Character {
        name: String::from(""),
        health: 0,
        alive: false,
    };

    let mut game_on = true;

    let mut player = Player {
        character: dummy_character,
        has_character: false,
        player_name: String::from(""),
    };

    println!("Welcome to the arena!");

    while game_on {
        println!("Player name: {}", player.character.name);
        if player.character.alive == false {
            player.character = characters::start_new_player_character(game_on);
        } else {
            println!("Your character is {}", player.character.name);
            fight::fight(&player.character);
            game_on = false;
        }
    }
}
