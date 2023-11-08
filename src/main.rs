#![allow(non_snake_case)]
use std::io;

mod Character;

fn main() {
    let 
    
    println!("input a line, yo");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Could not read input");

    
    let mut player = match input_text.as_str() {
        "Cat" => Character::new(
            Race::Cat, Class::Magical
        ),
        //"Demon" => Character::new(),
        //"Human" => Character::new(),
        _ => panic!("Invalid Input!"),
    };
}
