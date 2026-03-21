use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Crafting {
    // Glassblowing
    BeerGlass,
    Vial,
    Orb,
    // Battlestaffs
    WaterBattlestaff,
    EarthBattlestaff,
    FireBattlestaff,
    AirBattlestaff,
    // Pottery
    Pot,
    PieDish,
    Bowl,
    // Leather
    LeatherGloves,
    LeatherBoots,
    LeatherCowl,
    LeatherVambraces,
    LeatherBody,
    LeatherChaps,
    Coif,
    HardLeatherBody,
    StuddedLeatherBody,
    StuddedLeatherChaps,
    // Dragonhide
    GreenLeatherVambraces,
    GreenLeatherChaps,
    GreenLeatherBody,
    BlueLeatherVambraces,
    BlueLeatherChaps,
    BlueLeatherBody,
    RedLeatherVambraces,
    RedLeatherChaps,
    RedLeatherBody,
    BlackLeatherVambraces,
    BlackLeatherChaps,
    BlackLeatherBody,
    // Gem Cutting
    Sapphire,
    Emerald,
    Ruby,
    Diamond,
    Dragonstone,
    // Rings
    GoldRing,
    SapphireRing,
    EmeraldRing,
    RubyRing,
    DiamondRing,
    DragonstoneRing,
    // Necklaces
    GoldNecklace,
    SapphireNecklace,
    EmeraldNecklace,
    RubyNecklace,
    DiamondNecklace,
    DragonstoneNecklace,
    // Amulets
    GoldAmulet,
    SapphireAmulet,
    EmeraldAmulet,
    RubyAmulet,
    DiamondAmulet,
    DragonstoneAmulet,
}

impl Detail for Crafting {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Crafting(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Crafting(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Crafting(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Crafting {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Pot,
            Self::LeatherGloves,
            Self::BeerGlass,
            Self::GoldRing,
            Self::GoldNecklace,
            Self::PieDish,
            Self::LeatherBoots,
            Self::Bowl,
            Self::GoldAmulet,
            Self::LeatherCowl,
            Self::LeatherVambraces,
            Self::LeatherBody,
            Self::LeatherChaps,
            Self::Sapphire,
            Self::SapphireRing,
            Self::SapphireNecklace,
            Self::SapphireAmulet,
            Self::Emerald,
            Self::EmeraldRing,
            Self::HardLeatherBody,
            Self::EmeraldNecklace,
            Self::EmeraldAmulet,
            Self::Vial,
            Self::Ruby,
            Self::RubyRing,
            Self::Coif,
            Self::RubyNecklace,
            Self::StuddedLeatherBody,
            Self::StuddedLeatherChaps,
            Self::Diamond,
            Self::DiamondRing,
            Self::Orb,
            Self::RubyAmulet,
            Self::WaterBattlestaff,
            Self::DragonstoneRing,
            Self::Dragonstone,
            Self::DiamondNecklace,
            Self::GreenLeatherVambraces,
            Self::EarthBattlestaff,
            Self::GreenLeatherChaps,
            Self::FireBattlestaff,
            Self::GreenLeatherBody,
            Self::BlueLeatherVambraces,
            Self::AirBattlestaff,
            Self::BlueLeatherChaps,
            Self::DiamondAmulet,
            Self::BlueLeatherBody,
            Self::DragonstoneNecklace,
            Self::RedLeatherVambraces,
            Self::RedLeatherChaps,
            Self::RedLeatherBody,
            Self::BlackLeatherVambraces,
            Self::DragonstoneAmulet,
            Self::BlackLeatherChaps,
            Self::BlackLeatherBody,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::LeatherBody,
            Self::WaterBattlestaff,
            Self::EarthBattlestaff,
            Self::FireBattlestaff,
            Self::AirBattlestaff,
            Self::GreenLeatherBody,
            Self::BlueLeatherBody,
            Self::RedLeatherBody,
            Self::BlackLeatherBody,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            // Glassblowing
            Self::BeerGlass => ("Beer Glass", 3, 17.5),
            Self::Vial => ("Vial", 33, 35.0),
            Self::Orb => ("Orb", 46, 52.5),
            // Battlestaffs
            Self::WaterBattlestaff => ("Water Battlestaff", 54, 100.0),
            Self::EarthBattlestaff => ("Earth Battlestaff", 58, 112.5),
            Self::FireBattlestaff => ("Fire Battlestaff", 63, 125.0),
            Self::AirBattlestaff => ("Air Battlestaff", 66, 137.5),
            // Pottery
            Self::Pot => ("Pot", 1, 12.5),
            Self::PieDish => ("Pie Dish", 7, 25.0),
            Self::Bowl => ("Bowl", 8, 25.0),
            // Leather
            Self::LeatherGloves => ("Leather Gloves", 1, 13.8),
            Self::LeatherBoots => ("Leather Boots", 7, 16.3),
            Self::LeatherCowl => ("Leather Cowl", 9, 18.5),
            Self::LeatherVambraces => ("Leather Vambraces", 11, 22.0),
            Self::LeatherBody => ("Leather Body", 14, 25.0),
            Self::LeatherChaps => ("Leather Chaps", 18, 27.0),
            Self::Coif => ("Coif", 38, 37.0),
            Self::HardLeatherBody => ("Hard Leather Body", 28, 35.0),
            Self::StuddedLeatherBody => ("Studded Leather Body", 41, 65.0),
            Self::StuddedLeatherChaps => ("Studded Leather Chaps", 44, 69.0),
            // Dragonhide
            Self::GreenLeatherVambraces => ("Green Leather Vambraces", 57, 62.0),
            Self::GreenLeatherChaps => ("Green Leather Chaps", 60, 124.0),
            Self::GreenLeatherBody => ("Green Leather Body", 63, 186.0),
            Self::BlueLeatherVambraces => ("Blue Leather Vambraces", 66, 70.0),
            Self::BlueLeatherChaps => ("Blue Leather Chaps", 68, 140.0),
            Self::BlueLeatherBody => ("Blue Leather Body", 71, 210.0),
            Self::RedLeatherVambraces => ("Red Leather Vambraces", 73, 78.0),
            Self::RedLeatherChaps => ("Red Leather Chaps", 75, 156.0),
            Self::RedLeatherBody => ("Red Leather Body", 77, 234.0),
            Self::BlackLeatherVambraces => ("Black Leather Vambraces", 79, 86.0),
            Self::BlackLeatherChaps => ("Black Leather Chaps", 82, 172.0),
            Self::BlackLeatherBody => ("Black Leather Body", 84, 258.0),
            // Gem Cutting
            Self::Sapphire => ("Sapphire", 20, 50.0),
            Self::Emerald => ("Emerald", 27, 67.0),
            Self::Ruby => ("Ruby", 34, 85.0),
            Self::Diamond => ("Diamond", 43, 107.5),
            Self::Dragonstone => ("Dragonstone", 55, 137.5),
            // Rings
            Self::GoldRing => ("Gold Ring", 5, 15.0),
            Self::SapphireRing => ("Sapphire Ring", 8, 40.0),
            Self::EmeraldRing => ("Emerald Ring", 18, 55.0),
            Self::RubyRing => ("Ruby Ring", 34, 70.0),
            Self::DiamondRing => ("Diamond Ring", 43, 85.0),
            Self::DragonstoneRing => ("Dragonstone Ring", 55, 100.0),
            // Necklaces
            Self::GoldNecklace => ("Gold Necklace", 6, 20.0),
            Self::SapphireNecklace => ("Sapphire Necklace", 10, 55.0),
            Self::EmeraldNecklace => ("Emerald Necklace", 24, 60.0),
            Self::RubyNecklace => ("Ruby Necklace", 40, 75.0),
            Self::DiamondNecklace => ("Diamond Necklace", 56, 90.0),
            Self::DragonstoneNecklace => ("Dragonstone Necklace", 72, 105.0),
            // Amulets
            Self::GoldAmulet => ("Gold Amulet", 8, 30.0),
            Self::SapphireAmulet => ("Sapphire Amulet", 24, 65.0),
            Self::EmeraldAmulet => ("Emerald Amulet", 30, 70.0),
            Self::RubyAmulet => ("Ruby Amulet", 50, 85.0),
            Self::DiamondAmulet => ("Diamond Amulet", 70, 100.0),
            Self::DragonstoneAmulet => ("Dragonstone Amulet", 80, 150.0),
        };

        Details::Crafting(CraftingDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
        })
    }

    fn search<T>(query: T) -> Vec<Self>
    where
        T: ToString,
        Self: Sized,
    {
        let mut all = Self::all();

        let q = query.to_string().to_lowercase();

        if let Ok(pattern) = Regex::new(q.as_str()) {
            let mut index = 0;
            all.retain(|activity| {
                if pattern
                    .captures(activity.name().to_lowercase().as_str())
                    .iter()
                    .count()
                    > 0
                    && index < 10
                {
                    index = index.add(1);

                    return true;
                }

                return false;
            });
        } else {
            return vec![];
        }

        all
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct CraftingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
}

impl IntoString for CraftingDetails {
    fn to_string(&self, s: &crate::stats::skill::Source, xp_difference: f64) -> String {
        format!(
            "{}: {}",
            s.c1(self.name.as_str()),
            s.c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp).ceil()).as_str(),
                "d"
            )
            .as_str())
        )
    }
}
