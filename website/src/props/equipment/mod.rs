

mod weapon;
mod armor;

pub use self::{armor::*, weapon::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Equipment {
    name: String,
    weight: u32,
    properties: EquipmentProperties,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EquipmentProperties {
    Weapon(WeaponProperties),
    Armor(ArmorProperties),
    Shield(ShieldProperties),
}
