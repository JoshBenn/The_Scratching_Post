//============= Primary Structs =============
#[derive(Debug)]
pub struct Character {  //Character
    pub race: Race,
    pub class: Class,
    pub weapon: Weapon,
    pub armor_set: ArmorSet,
    pub health: f64,
    pub experience: u32,
    pub level: u8,
}

#[derive(Debug)]
pub struct Weapon {     //Weapon
    pub weapon_type: WeaponType,
    pub accuracy: i16,
    pub penetration: i16,
    pub power: u16,
}

#[derive(Debug)]
pub struct ArmorSet {  //ArmorSet
    pub helmet: Armor,
    pub body: Armor,
    pub legs: Armor,
    pub feet: Armor,
}

#[derive(Debug)]
pub struct Armor {     //Armor
    pub armor_type: ArmorType,
    pub accuracy: i16,
    pub penetration: i16,
}

//============== Enums ================
#[derive(Debug)]
pub enum Race {        //Race
    Cat,
    Demon,
    Human,
}

#[derive(Debug, Copy, Clone)]
pub enum Class {       //Class
    Physical,
    Magical,
    Ranged,
}

#[derive(Debug)]
pub enum WeaponType {  //WeaponType
    Staff,
    Sword,
    Bow,
}

#[derive(Debug)]
pub enum ArmorType {   //ArmorType
    Helmet,
    Body,
    Legs,
    Feet,
}

//=========== Implementations ===========
impl Character {               //Character
    pub fn new(race: Race, class: Class) -> Self {
        Self {
            race: race,
            class: class,
            armor_set: ArmorSet {..Default::default() },
            weapon: Weapon::new(
                match class {
                    Class::Physical => WeaponType::Sword,
                    Class::Magical => WeaponType::Staff,
                    Class::Ranged => WeaponType::Bow,
                }, (1, 1, 1)
            ),
            health: 10.0,
            experience: 0,
            level: 1,
        }
    }
}

impl Default for Character {  //Character Default
    fn default() -> Self {
        Self {
            race: Race::Cat,
            class: Class::Magical,
            armor_set: ArmorSet { ..Default::default() },
            weapon: Weapon::new(WeaponType::Staff, (1, 1, 1)),
            health: 10.0,
            experience: 0,
            level: 1, 
        }
    }
}

impl Weapon {                //Weapon
    pub fn new(weapon_type: WeaponType, stats: (i16, i16, u16)) -> Self {
        Self {
            weapon_type,
            accuracy: stats.0,
            penetration: stats.1,
            power: stats.2,
        }
    }
}

impl ArmorSet {               //ArmorSet
    //TODO
}

impl Default for  ArmorSet {  //ArmorSet Default
    fn default() -> Self {
        Self {
            helmet: Armor::new(ArmorType::Helmet, (1, 1)),
            body: Armor::new(ArmorType::Body, (1, 1)),
            legs: Armor::new(ArmorType::Legs, (1, 1)),
            feet: Armor::new(ArmorType::Feet, (1, 1)),
        }
    }
}

impl Armor {                  //Armor
    pub fn new(armor_type: ArmorType, stats: (i16, i16)) -> Self {
        Self {
            armor_type,
            accuracy: stats.0,
            penetration: stats.1,
        }
    }
}




#[config(test)]
mod test_character {
    //add testing functionality
}
