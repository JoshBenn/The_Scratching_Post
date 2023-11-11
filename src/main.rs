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
            Err(error) => println!("Erro: {}", error),
        }
    }

    let mut player = player.unwrap();
  
    println!("{:#?}", player);
}



fn create_character(race: &str, class: &str) -> Result<Character, String> {
    let invalid_race = "Invalid race!\n\n".to_string();
    let invalid_class = "Invalid class\n\n".to_string();
    
    let player = match race {
        "Demon" => match class {
            "Physical" => Character::new(
                Race::Demon, Class::Physical
            ),
            "Magical" => Character::new(
                Race::Demon, Class::Magical
            ),
            "Ranged" => Character::new(
                Race::Demon, Class::Ranged
            ),
            _ => return Err(invalid_class),
        },
        "Cat" => match class {
            "Physical" => Character::new(
                Race::Cat, Class::Physical
            ),
            "Magical" => Character::new(
                Race::Cat, Class::Magical
            ),
            "Ranged" => Character::new(
                Race::Cat, Class::Ranged
            ),
            _ => return Err(invalid_class),
        },
        "Human" => match class {
            "Physical" => Character::new(
                Race::Human, Class::Physical
            ),
            "Magical" => Character::new(
                Race::Human, Class::Magical
            ),
            "Ranged" => Character::new(
                Race::Human, Class::Ranged
            ),
            _ => return Err(invalid_class),
        },
        _ => return Err(invalid_race),
    };
    Ok(player)
}
