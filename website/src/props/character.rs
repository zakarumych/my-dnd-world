use dioxus::prelude::*;

use crate::Route;
use std::{collections::HashMap, str::FromStr};

use super::Equipment;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Morality {
    Good,
    Neutral,
    Evil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidMorality;

impl FromStr for Morality {
    type Err = InvalidMorality;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Good" => Ok(Morality::Good),
            "Neutral" => Ok(Morality::Neutral),
            "Evil" => Ok(Morality::Evil),
            _ => Err(InvalidMorality),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Order {
    Lawful,
    Neutral,
    Chaotic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidOrder;

impl FromStr for Order {
    type Err = InvalidOrder;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lawful" => Ok(Order::Lawful),
            "Neutral" => Ok(Order::Neutral),
            "Chaotic" => Ok(Order::Chaotic),
            _ => Err(InvalidOrder),
        }
    }
}

mod alignment {
    use std::{fmt, str::FromStr};

    use serde::{
        de::{self, Deserialize, Deserializer, Visitor},
        ser::{Serialize, SerializeTuple, Serializer},
    };

    use super::{Morality, Order};

    // Custom serialization for alignment
    pub fn serialize<S>(alignment: &(Order, Morality), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let alignment_str = format!("{:?} {:?}", alignment.0, alignment.1);
            serializer.serialize_str(&alignment_str)
        } else {
            let mut serializer = serializer.serialize_tuple(2)?;
            serializer.serialize_element(&alignment.0)?;
            serializer.serialize_element(&alignment.1)?;
            serializer.end()
        }
    }

    // Custom deserialization for alignment
    pub fn deserialize<'de, D>(deserializer: D) -> Result<(Order, Morality), D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AlignmentVisitor;

        impl<'de> Visitor<'de> for AlignmentVisitor {
            type Value = (Order, Morality);

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a valid alignment string like 'Chaotic Evil' or a tuple of (Order, Morality)",
                )
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut parts = value.split_whitespace();
                let order_part = parts
                    .next()
                    .ok_or_else(|| de::Error::custom("Missing order"))?;
                let morality_part = parts
                    .next()
                    .ok_or_else(|| de::Error::custom("Missing morality"))?;

                let order =
                    Order::from_str(order_part).map_err(|_| de::Error::custom("Invalid order"))?;
                let morality = Morality::from_str(morality_part)
                    .map_err(|_| de::Error::custom("Invalid morality"))?;

                Ok((order, morality))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let order: Order = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Missing order"))?;
                let morality: Morality = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Missing morality"))?;
                Ok((order, morality))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(AlignmentVisitor)
        } else {
            deserializer.deserialize_tuple(2, AlignmentVisitor)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Species {
    Human,
    Elf,
    Dwarf,
    Orc,
    Halfling,
    Gnome,
    Tiefling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Class {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Wizard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub name: String,
    pub species: Species,

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    classes: HashMap<Class, u32>,

    #[serde(with = "alignment")]
    pub alignment: (Order, Morality),

    pub origin: String,

    pub r#str: u32,
    pub dex: u32,
    pub con: u32,
    pub int: u32,
    pub wis: u32,
    pub cha: u32,
    // pub max_hp: u16,

    // #[serde(skip_serializing_if = "is_zero", default)]
    // pub wealth: u32,

    // #[serde(skip_serializing_if = "Vec::is_empty", default)]
    // pub inventory: Vec<(String, u32)>,

    // pub age: u16,
    // pub sex: Sex,

    // #[serde(default)]
    // pub height: f32,

    // #[serde(default)]
    // pub weight: f32,

    // pub notes: String,
}

fn is_zero(value: &u32) -> bool {
    *value == 0
}

impl Character {
    pub fn new() -> Self {
        Character {
            name: "<unnamed>".to_string(),
            species: Species::Human,
            classes: HashMap::new(),
            alignment: (Order::Neutral, Morality::Neutral),
            origin: "<not chosen>".to_string(),
            r#str: 10,
            dex: 10,
            con: 10,
            int: 10,
            wis: 10,
            cha: 10,
            // max_hp: 0,
            // wealth: 0,
            // inventory: Vec::new(),
            // age: 0,
            // sex: Sex::Male,
            // height: 0.0,
            // weight: 0.0,
            // personality_traits: Vec::new(),
            // ideals: Vec::new(),
            // bonds: Vec::new(),
            // flaws: Vec::new(),
            // notes: String::new(),
        }
    }

    pub fn total_level(&self) -> u32 {
        self.classes.values().sum::<u32>()
    }
}
