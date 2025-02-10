use crate::props::RollModifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Armor {
    name: String,
    weight: u32,
    cost: u32,

    #[serde(flatten)]
    properties: ArmorProperties,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ArmorProperties {
    category: ArmorCategory,
    armor_class: u32,
    r#str: u32,
    stealth: Option<RollModifier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ArmorCategory {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Shield {
    name: String,
    weight: u32,
    cost: u32,

    #[serde(flatten)]
    properties: ShieldProperties,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ShieldProperties{
    armor_class: u32,
}
