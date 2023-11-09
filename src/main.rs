mod Character;

use std::io;
use Character::{Character as Char, Race, Class};


fn main() {
    println!("input a line, yo");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Could not read input");

    let input_text = input_text.trim();
    
    let mut player = match input_text {
        "Cat" => Char::new(
            Race::Cat, Class::Magical
        ),
        //"Demon" => Character::new(),
        //"Human" => Character::new(),
        _ => panic!("Invalid Input!"),
    };
}
