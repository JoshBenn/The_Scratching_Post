
#[derive(Debug)]
pub struct Character {
    pub race: Race,
    pub class: Class,
    pub health: f64,
    pub experience: u32,
    pub level: u8,
}

#[derive(Debug)]
pub enum Race {
    Cat,
    Demon,
    Human,
}

#[derive(Debug)]
pub enum Class {
    Physical,
    Magical,
    Ranged,
}

impl Character {
    pub fn new(race: Race, class: Class) -> Character {
        Character {
            race: race,
            class: class,
            health: 10.0,
            experience: 0,
            level: 1,
        }
    }
}

