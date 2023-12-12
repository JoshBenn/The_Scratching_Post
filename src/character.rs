use super::floor::*;
use rand::Rng;
use std::fmt::{self, Formatter, Display};


///Determines the type of damage a weapon does
#[derive(Debug)]
pub enum DamageType {
    Physical,
    Magical,
}

///Display for DamageType
impl Display for DamageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DamageType::Physical => write!(f, "Physical"),
            DamageType::Magical => write!(f, "Magical"),
        }
    }
}

///The type of weapon for a character
//TODO: Expand different weapon types
#[derive(Debug)]
pub enum Weapon {
    ///Physical DamageType
    Sword(WeaponStats),
    ///Magical DamageType
    Staff(WeaponStats),
    ///Physical DamageType
    Bow(WeaponStats),
}

///Implementation for weapon
impl Weapon {
    ///Creates a new sword based on stats provided
    ///Parameters: (power: u16, accuracy: i16, penetration: i16)
    pub fn new_sword(power: u16, penetration: i16, accuracy: i16) -> Self {
        Weapon::Sword(
            WeaponStats::new(DamageType::Physical, power, penetration, accuracy),
        )
    }

    ///Creates a new staff based on stats provided
    ///Parameters: (power: u16, accuracy: i16, penetration: i16)
    pub fn new_staff(power: u16, penetration: i16, accuracy: i16) -> Self {
        Weapon::Staff(
            WeaponStats::new(DamageType::Magical, power, penetration, accuracy),
        )
    }

    ///Creates a new bow based on stats provided
    ///Parameters: (power: u16, accuracy: i16, penetration: i16)
    pub fn new_bow(power: u16, penetration: i16, accuracy: i16) -> Self {
        Weapon::Bow(
            WeaponStats::new(DamageType::Physical, power, penetration, accuracy),
        )
    }

    ///Generates a new weapon with the stats provided
    ///Parameters: (power: u16, accuracy: i16, penetration: i16)
    pub fn new_random_weapon(power: u16, penetration: i16, accuracy: i16) -> Self {
        let mut rng = rand::thread_rng();

        match rng.gen_range(1..=3) {
            1 => Weapon::new_sword(power, penetration, accuracy),
            2 => Weapon::new_staff(power, penetration, accuracy),
            3 => Weapon::new_bow(power, penetration, accuracy),
            _ => unreachable!(),
        }
    } 
}

impl Display for Weapon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let weapon_stats = match self {
            Weapon::Sword(stats) => stats,
            Weapon::Staff(stats) => stats,
            Weapon::Bow(stats) => stats,
        };

        todo!()
    }
}

///Weapon determines the damage type
#[derive(Debug)]
pub struct WeaponStats {
    ///Magical or Physical
    pub damage_type: DamageType,
    ///Strength of the weapon
    pub power: u16,
    ///Percent chance to hit
    pub accuracy: i16,
    ///Percent chance to hit for full power
    ///Value from 0 to 1 -> multiplies the power
    pub penetration: i16,
}

///Implementation for WeaponStats
impl WeaponStats {
    ///Generates a new weaponstats struct based on the stats provided
    ///Damage type: Magical or Physical
    ///power: u16, penetration: i16, accuracy: i16
    pub fn new(damage_type: DamageType, power: u16, penetration: i16, accuracy: i16) -> Self {
        Self {
            damage_type,
            power,
            penetration,
            accuracy,
        }
    }
}

impl Display for WeaponStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

///Holds the primary stats for any armor type
#[derive(Debug)]
pub struct ArmorStats {
    ///Chance to hit
    pub accuracy: i16,
    ///Multiplier to increase base power
    pub penetration: i16,
    ///Defense against physical attacks
    pub physical_defense: i16,
    ///Defense againt magical attacks
    pub magical_defense: i16,
}

impl ArmorStats {
    pub fn new(accuracy: i16, penetration: i16, physical_defense: i16, magical_defense: i16) -> Self {
        Self {
            accuracy,
            penetration,
            physical_defense,
            magical_defense,
        }
    }
}

///Each armor type will contain their resective armor stats
#[derive(Debug)]
pub enum ArmorType {
    Helmet(ArmorStats),
    Body(ArmorStats),
    Gloves(ArmorStats),
    Legs(ArmorStats),
    Feet(ArmorStats),
}

///Implementation for all armor types
impl ArmorType {
    ///Generate a new helmet based on the stats passed
    pub fn new_helmet(accuracy: i16, penetration: i16, physical_defense: i16, magical_defense: i16) -> Self {
        ArmorType::Helmet (
            ArmorStats::new(accuracy, penetration, physical_defense, magical_defense
        ))
    }

    ///Generate a new body based on the stats passed
    pub fn new_body(accuracy: i16, penetration: i16, physical_defense: i16, magical_defense: i16) -> Self {
        ArmorType::Body (
            ArmorStats::new(accuracy, penetration, physical_defense, magical_defense
        ))
    }

    ///Generate new gloves based on the stats passed
    pub fn new_gloves(accuracy: i16, penetration: i16, physical_defense: i16, magical_defense: i16) -> Self {
        ArmorType::Gloves (
            ArmorStats::new(accuracy, penetration, physical_defense, magical_defense
        ))
    }

    ///Generate new leggings based on the stats passed
    pub fn new_legs(accuracy: i16, penetration: i16, physical_defense: i16, magical_defense: i16) -> Self {
        ArmorType::Legs (
            ArmorStats::new(accuracy, penetration, physical_defense, magical_defense
        ))
    }

    ///Generate new boots based on the stats passed
    pub fn new_feet(accuracy: i16, penetration: i16, physical_defense: i16, magical_defense: i16) -> Self {
        ArmorType::Feet (
            ArmorStats::new(accuracy, penetration, physical_defense, magical_defense
        ))
    }

    ///Generate a new random armor piece
    //TODO: Find a better method for stat allocation to keep them unique
    pub fn new_random(floor_type: FloorType) -> Self {
        let floor = match floor_type {
            FloorType::Small(floor) => floor,
            FloorType::Medium(floor) => floor,
            FloorType::Large(floor) => floor,
        };
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=5) {
            1 => { ArmorType::new_helmet(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            )},
            2 => { ArmorType::new_body(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            )},
            3 => { ArmorType::new_gloves(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            )},
            4 => { ArmorType::new_legs(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            )},
            5 => { ArmorType::new_feet(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            )},
            _ => unreachable!(),
        }
    }
}


///Consists of every armor type for any character
#[derive(Debug)]
pub struct ArmorSet {
    pub helmet: ArmorType,
    pub body: ArmorType,
    pub gloves: ArmorType,
    pub legs: ArmorType,
    pub feet: ArmorType,
}

///Implementation for the armor set
impl ArmorSet {
    ///Create a new armor set
    pub fn new(helmet: ArmorType, body: ArmorType, gloves: ArmorType, legs: ArmorType, feet: ArmorType) -> Self {
        Self {
            helmet,
            body,
            gloves,
            legs,
            feet,
        }
    }

    pub fn new_random(floor_type: FloorType) -> Self {
        let floor = match floor_type {
            FloorType::Small(floor) => floor,
            FloorType::Medium(floor) => floor,
            FloorType::Large(floor) => floor,
        };
        Self {
            helmet: ArmorType::new_helmet(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            ),
            body: ArmorType::new_body(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            ),
            gloves: ArmorType::new_gloves(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            ),
            legs: ArmorType::new_legs(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            ),
            feet: ArmorType::new_feet(
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
                floor.level as i16,
            ),
        }
    }
}

///Default implementation for the armor set
impl Default for ArmorSet {
    ///ArmorType parameters: accuracy, penetration, phys_defense, magic_defense
    fn default() -> Self {
        Self {
            helmet: ArmorType::new_helmet(-1, 1, 1, 1),
            body: ArmorType::new_body(0, 1, 1, 1),
            gloves: ArmorType::new_gloves(1, 1, 1, 1),
            legs: ArmorType::new_legs(0, 1, 1, 1),
            feet: ArmorType::new_feet(0, 1, 1, 1),
        }
    }
}

///Primary enum representing the character object
#[derive(Debug)]
pub enum Character {
    ///Cat Traits:
    /// Positives:
    /// - High crit (+0.2x value)
    /// Negatives:
    /// - Low defense (-0.2x value)
    Cat(CharacterTraits),
    ///Demon Traits:
    /// Positives:
    /// - High defense (+0.2x value)
    /// Negatives:
    /// - Low Power (-0.2x value)
    Demon(CharacterTraits),
    ///Human Traits:
    /// Positives:
    /// - High Health (+0.2x value)
    /// Negatives:
    /// - None
    Human(CharacterTraits),
}

///Implementation for Character
impl Character {
    ///Generates a new Cat character
    pub fn new_cat(weapon: Weapon) -> Character {
        Character::Cat (
            CharacterTraits::new(weapon),
        )
    }

    ///Generates a new Demon character
    pub fn new_demon(weapon: Weapon) -> Character {
        Character::Demon (
            CharacterTraits::new(weapon),
        )
    }

    ///Generates a new Human character
    pub fn new_human(weapon: Weapon) -> Character {
        Character::Human (
            CharacterTraits::new(weapon),
        )
    }

    ///Generates a new random character based on floor provided
    pub fn new_random(floor_type: FloorType) -> Character {
        //Random number to determine the character type
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=3) { //Generate the character
            1 => {
               Character::Cat(CharacterTraits::new_random(floor_type)) 
            },
            2 => {
                Character::Demon(CharacterTraits::new_random(floor_type))
            },
            3 => {
                Character::Human(CharacterTraits::new_random(floor_type))
            },
            _ => unreachable!(),
        }
    }

}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (race, traits) = match self {
            Character::Cat(traits) => ("Cat", traits),
            Character::Demon(traits) => ("Demon", traits),
            Character::Human(traits) => ("Human", traits),
        };
        let weapon_stats = match &traits.weapon {
            Weapon::Sword(stats) => stats,
            Weapon::Staff(stats) => stats,
            Weapon::Bow(stats) => stats,
        };
        let accuracy = 0;
        let penetration = 0;
        let power = 0;
        let physical_def = 0;
        let magical_def = 0;
        
        write!(f, "{}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}",
            format!("Level {} {}:", traits.level, race),
            format!("Health: {}:", traits.health),
            format!("Experience: {}:", traits.experience),
            format!("Accuracy: {}:", accuracy),
            format!("Penetration: {}:", penetration),
            format!("Damage Type: {}:", weapon_stats.damage_type),
            format!("Power: {}:", power),
            format!("Physical Def: {}:", physical_def),
            format!("Magical Def: {}:", magical_def),
        )
        
    }
}

///Struct which holds all of the general information for the character
#[derive(Debug)]
pub struct CharacterTraits {
    ///Health ranges from 0 to xxxx //TODO
    pub health: f64,
    ///Experience will be used to level up and has a cap of xxx per level //TODO
    pub experience: u64,
    ///Level is based on the experience gained after every fight
    ///Max level is xxxx //TODO
    pub level: u8,
    ///Weapon is currently limited to 3 weapon types
    //TODO: Expand weapon types
    //TODO: Off-hand?
    pub weapon: Weapon,
    ///Armor set contains each armor piece
    ///Head, Chest, Gloves, Legs, Feet
    pub armor_set: ArmorSet,
}

///Implementation for character traits
impl CharacterTraits {
    ///Default character traits
    pub fn new(weapon: Weapon) -> Self {
        Self {
            armor_set: ArmorSet {
                ..Default::default()
            },
            weapon,
            health: 10.0,
            experience: 0,
            level: 1,
        }
    }

    ///Generate a random new character based on the floor information
    pub fn new_random(floor_type: FloorType) -> Self {
        let mut rng = rand::thread_rng();
        //TODO: Make this better to generate better stats
        let floor = match floor_type {
                FloorType::Small(floor) => floor,
                FloorType::Medium(floor) => floor,
                FloorType::Large(floor) => floor,
        };

        Self {
            weapon: Weapon::new_random_weapon(
                floor.level, 
                floor.level as i16, 
                floor.level as i16
            ),
            armor_set: ArmorSet::new_random(floor_type),
            health: floor.level as f64,
            experience: 0,
            level: floor.level as u8,
        }
    }
}
