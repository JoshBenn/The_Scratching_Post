mod character;

use std::io;
use character::{Character, Race, Class};


fn main() {
    println!("input a line, yo");
    let mut race_choice = String::new();
    let mut class_choice = String::new();
    io::stdin()
        .read_line(&mut race_choice)
        .expect("Could not read race input!");
    io::stdin()
        .read_line(&mut class_choice)
        .expect("Could not read class input!");

    let race_choice = race_choice.trim();
    let class_choice = class_choice.trim();

    let mut player = create_character(race_choice, class_choice);
  
    println!("{:#?}", player);
}



fn create_character(race: &str, class: &str) -> Character {
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
            _ => todo!(),
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
            _ => todo!(),
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
            _ => todo!(),
        },
        _ => todo!(),
    };
    player
}
