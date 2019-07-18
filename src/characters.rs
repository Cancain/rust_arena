use std::io;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub health: i32,
    pub alive: bool,
}


pub fn start_new_player_character() -> Character {
    let mut name = String::from("");

    println!("Seems as if you have no character created yet, please enter the name of a new character");
    while name == "" {
        io::stdin()
            .read_line(&mut name)
            .expect("Something went wrong, try again");
    }

  let character = Character {
      name: name,
      health: 100,
      alive: true,
  }; 

  character
}
