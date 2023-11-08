#![allow(non_snake_case)]
mod Character;

use std::io;
use Character::{Race, Class};


fn main() {
    let 
    
    println!("input a line, yo".to_string());
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Could not read input");

    let input_text = input_text.trim();
    
    let mut player = match input_text {
        "Cat" => Character::new(
            Race::Cat, Class::Magical
        ),
        //"Demon" => Character::new(),
        //"Human" => Character::new(),
        _ => panic!("Invalid Input!"),
    };
}
