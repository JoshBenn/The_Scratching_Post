#![allow(non_snake_case)]
use std::io;

mod Character;

fn main() {
    println!("input a line, yo");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Could not read input");

    
        println!("Your input: {}", input_text);
}
