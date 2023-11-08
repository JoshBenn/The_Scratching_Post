pub struct Character {
    race: Race,
    class: Class,
    health: f64,
    experience: u32,
    level: u8,
}

enum Race {
    Cat,
    Demon,
    Human,
}

enum Class {
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

