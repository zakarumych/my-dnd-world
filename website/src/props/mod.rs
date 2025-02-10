
mod character;
mod equipment;
mod damage;

pub use self::{
    character::*,
    equipment::*,
    damage::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RollValue {
    flat: u32,
    dices: Vec<(u32, Dice)>,    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RollModifier {
    Advantage,
    Disadvantage,
}
