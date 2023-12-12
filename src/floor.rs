

pub enum FloorType {
    Small(Floor),
    Medium(Floor),
    Large(Floor),
}

#[derive(Clone, Copy)]
pub struct Floor {
    pub level: u16,
    pub num_mobs: u8,
}

impl Floor {
    pub fn new(level: u16) -> Self {
        Self {
            level,
            num_mobs: 1,
        }
    }
}
