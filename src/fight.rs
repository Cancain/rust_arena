use crate::characters::Character;

pub fn fight(player: &Character) {
    let enemy_npc = crate::characters::generate_enemy_npc();
    println!("Get ready to fight");
    println!("{} vs {}", player.name, enemy_npc.name );

}