mod character;

use std::io;
use character::{Character, Race, Class};


fn main() {
    //Get the input from the player
    let mut player: Option<Character> = None;
    //Loop until a proper input is provided
    while player.is_none() {
        let mut race_choice = String::new();
        let mut class_choice = String::new();

        println!("\nInput a race\n==[Cat] : [Demon] : [Human]==");
        io::stdin()
            .read_line(&mut race_choice)
            .expect("Could not read race input!\n\n");
        println!("\nInput a class\n==[Physical] : [Magical] : [Ranged]==");
        io::stdin() 
            .read_line(&mut class_choice)
            .expect("Could not read class input!\n\n");
        
        let race_choice = race_choice.trim();
        let class_choice = class_choice.trim();

        match create_character(race_choice, class_choice) {
            Ok(created) => player = Some(created),
            Err(error) => println!("Error: {}", error),
        }
    }

    let mut player = player.unwrap();
  
    println!("{:#?}", player);
}



fn create_character(race: &str, class: &str) -> Result<Character, String> {
    //Grab the race choice
    let player_race = match race {
        "Cat" => Race::Cat,
        "Demon" => Race::Demon,
        "Human" => Race::Human,
        _ => return Err(String::from("Invalid race!")),
    };
    //Return the newly created character
    return match class {
        "Magical" => Ok(Character::new(player_race, Class::Magical)),
        "Physical" => Ok(Character::new(player_race, Class::Physical)),
        "Ranged" => Ok(Character::new(player_race, Class::Ranged)),
        _=> return Err(String::from("Invalid class!")),
    };
}
