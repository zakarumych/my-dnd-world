use crate::props::Damage;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Weapon {
    name: String,
    weight: u32,
    cost: u32,

    #[serde(flatten)]
    properties: WeaponProperties,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct WeaponProperties {
    damage: Damage,
    category: WeaponCategory,
    ammunition: Option<Ammunition>,
    finesse: bool,
    heavy: bool,
    light: bool,
    loading: bool,
    range: Option<(u32, u32)>,
    reach: bool,
    thrown: bool,
    two_handed: bool,
    versatile: Option<Damage>,
    mastery: Option<MasteryProperty>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WeaponCategory {
    Simple,
    Martial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Ammunition {
    Arrow,
    Bolt,
    Bullet,
    Dart,
    SlingBullet,
    BlowgunNeedle,
    Net,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MasteryProperty {
    Cleave,
    Graze,
    Nick,
    Push,
    Sap,
    Slow,
    Topple,
    Vex,
}
