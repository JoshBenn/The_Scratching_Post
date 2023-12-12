mod character;
mod floor;

use std::io;
use floor::*;
use character::{Character, Weapon};


fn main() {
    //Get the input from the player
    let mut player: Option<Character> = None;
    //Loop until a proper input is provided
    while player.is_none() {
        let mut race_choice = String::new();
        let mut weapon_choice = String::new();
        //Query player -> get inputs
        println!("\nInput a race\n==[Cat] : [Demon] : [Human]==");
        io::stdin()
            .read_line(&mut race_choice)
            .expect("Could not read race input!\n\n");
        println!("\nInput a starting weapon\n==[Sword] : [Staff] : [Bow]==");
        io::stdin() 
            .read_line(&mut weapon_choice)
            .expect("Could not read class input!\n\n");
        //Trim excess
        let race_choice = race_choice.trim();
        let weapon_choice = weapon_choice.trim();
        //Create the character
        match create_player(race_choice.to_lowercase(), weapon_choice.to_lowercase()) {
            Ok(created) => player = Some(created),
            Err(error) => println!("Error: {}", error),
        }
    }
    
    let player = player.unwrap();

    let enemy = character::Character::new_random(
        FloorType::Small(Floor::new(1))
    );
  
    println!("{}", player);
    println!("{}", enemy);

}


///Creates the player character
fn create_player(race: String, weapon: String) -> Result<Character, String> {
    //Get player weapon choice
    let player_weapon = match weapon.as_str() {
        "sword" => Weapon::new_sword(1, 1, 1),
        "staff" => Weapon::new_staff(1, 1, 1),
        "bow" => Weapon::new_bow(1, 1, 1),
        _ => return Err(String::from("Invalid race!")),
    };
    //Return the newly created character
    return match race.as_str() {
        "cat" => Ok(Character::new_cat(player_weapon)),
        "demon" => Ok(Character::new_demon(player_weapon)),
        "human" => Ok(Character::new_human(player_weapon)),
        _=> return Err(String::from("Invalid class!")),
    };
}
