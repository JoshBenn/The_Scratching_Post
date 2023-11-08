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


