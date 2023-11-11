
#[derive(Debug)]
pub struct Character {
    pub race: Race,
    pub class: Class,
    pub weapon: Weapon,
    pub armor_set: ArmorSet,
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

#[derive(Debug, Copy, Clone)]
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
            armor_set: ArmorSet {..Default::default() },
            weapon: Weapon::new(
                match class {
                    Class::Physical => WeaponType::Sword,
                    Class::Magical => WeaponType::Staff,
                    Class::Ranged => WeaponType::Bow,
                }, 1, 1, 1
            ),
            health: 10.0,
            experience: 0,
            level: 1,
        }
    }
}


impl Default for Character {
    fn default() -> Character {
        Character {
            race: Race::Cat,
            class: Class::Magical,
            armor_set: ArmorSet { ..Default::default() },
            weapon: Weapon::new(WeaponType::Staff, 1, 1, 1),
            health: 10.0,
            experience: 0,
            level: 1, 
        }
    }
}

#[derive(Debug)]
pub struct ArmorSet {
    pub helmet: Armor,
    pub body: Armor,
    pub legs: Armor,
    pub feet: Armor,
}

impl Default for  ArmorSet {
    fn default() -> ArmorSet {
        ArmorSet {
            helmet: Armor::new(ArmorType::Helmet, 1, 1),
            body: Armor::new(ArmorType::Body, 1, 1),
            legs: Armor::new(ArmorType::Legs, 1, 1),
            feet: Armor::new(ArmorType::Feet, 1, 1),
        }
    }
}

#[derive(Debug)]
pub struct Armor {
    pub armor_type: ArmorType,
    pub accuracy: i16,
    pub penetration: i16,
}

#[derive(Debug)]
pub enum ArmorType {
    Helmet,
    Body,
    Legs,
    Feet,
}

impl Armor {
    pub fn new(armor_type: ArmorType, accuracy: i16, penetration: i16) -> Armor {
        Armor {
            armor_type: armor_type,
            accuracy: accuracy,
            penetration: penetration,
        }
    }
}


#[derive(Debug)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub accuracy: i16,
    pub penetration: i16,
    pub power: u16,
}

#[derive(Debug)]
pub enum WeaponType {
    Staff,
    Sword,
    Bow,
}

impl Weapon {
    pub fn new(weapon_type: WeaponType, accuracy: i16, penetration: i16, power: u16) -> Weapon {
        Weapon {
            weapon_type: weapon_type,
            accuracy: accuracy,
            penetration: penetration,
            power: power,
        }
    }
}


