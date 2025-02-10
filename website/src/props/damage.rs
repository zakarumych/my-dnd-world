use super::RollValue;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DamageType {
    Acid, // Corrosive liquids, digestive enzymes
    Bludgeoning, // Blunt objects, constriction, falling
    Cold, // Freezing water, icy blasts
    Fire, // Flames, unbearable heat
    Force, // Pure magical energy
    Lightning, // Electricity
    Necrotic, // Life-draining energy
    Piercing, // Fangs, puncturing objects
    Poison, // Toxic gas, venom
    Psychic, // Mind-rending energy
    Radiant, // Holy energy, searing radiation
    Slashing, // Claws, cutting objects
    Thunder, // Concussive sound
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Damage {
    rolls: Vec<(DamageType, RollValue)>,
}
