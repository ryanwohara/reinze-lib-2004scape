use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Herblore {
    // Herb Identification
    GuamLeaf,
    Marrentill,
    Tarromin,
    Harralander,
    RanarrWeed,
    IritLeaf,
    Avantoe,
    Kwuarm,
    Cadantine,
    Lantadyme,
    DwarfWeed,
    Torstol,
    // Potions
    AttackPotion,
    Antipoison,
    StrengthPotion,
    RestorePotion,
    DefencePotion,
    AgilityPotion,
    PrayerPotion,
    SuperAttack,
    SuperAntipoison,
    FishingPotion,
    SuperStrength,
    WeaponPoison,
    SuperRestore,
    SuperDefence,
    AntifirePotion,
    RangingPotion,
    ZamorakBrew,
}

impl Detail for Herblore {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Herblore(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Herblore(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Herblore(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Herblore {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::GuamLeaf,
            Self::AttackPotion,
            Self::Antipoison,
            Self::Marrentill,
            Self::Tarromin,
            Self::StrengthPotion,
            Self::Harralander,
            Self::RestorePotion,
            Self::RanarrWeed,
            Self::DefencePotion,
            Self::AgilityPotion,
            Self::PrayerPotion,
            Self::IritLeaf,
            Self::SuperAttack,
            Self::Avantoe,
            Self::SuperAntipoison,
            Self::FishingPotion,
            Self::Kwuarm,
            Self::SuperStrength,
            Self::WeaponPoison,
            Self::SuperRestore,
            Self::Cadantine,
            Self::SuperDefence,
            Self::Lantadyme,
            Self::AntifirePotion,
            Self::DwarfWeed,
            Self::RangingPotion,
            Self::Torstol,
            Self::ZamorakBrew,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::StrengthPotion,
            Self::PrayerPotion,
            Self::SuperAttack,
            Self::SuperStrength,
            Self::SuperRestore,
            Self::SuperDefence,
            Self::AntifirePotion,
            Self::RangingPotion,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            // Herb Identification
            Self::GuamLeaf => ("Guam Leaf", "", "", 3, 2.5),
            Self::Marrentill => ("Marrentill", "", "", 5, 3.75),
            Self::Tarromin => ("Tarromin", "", "", 11, 5.0),
            Self::Harralander => ("Harralander", "", "", 20, 6.0),
            Self::RanarrWeed => ("Ranarr Weed", "", "", 25, 7.5),
            Self::IritLeaf => ("Irit Leaf", "", "", 40, 9.0),
            Self::Avantoe => ("Avantoe", "", "", 48, 10.0),
            Self::Kwuarm => ("Kwuarm", "", "", 54, 11.25),
            Self::Cadantine => ("Cadantine", "", "", 65, 12.75),
            Self::Lantadyme => ("Lantadyme", "", "", 67, 13.0),
            Self::DwarfWeed => ("Dwarf Weed", "", "", 70, 13.75),
            Self::Torstol => ("Torstol", "", "", 76, 15.0),
            // Potions
            Self::AttackPotion => ("Attack", "Guam Leaf", "Eye of Newt", 3, 25.0),
            Self::Antipoison => ("Anti Poison", "Marrentill", "Ground Unicorn Horn", 5, 38.0),
            Self::StrengthPotion => ("Strength", "Tarromin", "Limpwurt Root", 12, 50.0),
            Self::RestorePotion => ("Restore", "Harralander", "Red Spiders' Egg", 22, 62.5),
            Self::DefencePotion => ("Defense", "Ranarr Weed", "White Berries", 30, 75.0),
            Self::AgilityPotion => ("Agility", "Toadflax", "Toad's Legs", 34, 80.0),
            Self::PrayerPotion => ("Restore Prayer", "Ranarr Weed", "Snape Grass", 38, 87.5),
            Self::SuperAttack => ("Super Attack", "Irit Leaf", "Eye of Newt", 45, 100.0),
            Self::SuperAntipoison => (
                "Super Anti Poison",
                "Irit Leaf",
                "Ground Unicorn Horn",
                48,
                106.0,
            ),
            Self::FishingPotion => ("Fishing", "Avantoe", "Snape Grass", 50, 113.0),
            Self::SuperStrength => ("Super Strength", "Kwuarm", "Limpwurt Root", 55, 125.0),
            Self::WeaponPoison => (
                "Weapon Poison",
                "Kwuarm",
                "Ground Blue Dragon Scale",
                60,
                137.5,
            ),
            Self::SuperRestore => ("Super Restore", "Snapdragon", "Red Spiders' Egg", 63, 142.0),
            Self::SuperDefence => ("Super Defense", "Cadantine", "White Berries", 66, 150.0),
            Self::AntifirePotion => (
                "Anti-FireBreath",
                "Lantadyme",
                "Ground Blue Dragon Scale",
                69,
                158.0,
            ),
            Self::RangingPotion => ("Ranging", "Dwarf Weed", "Wine of Zamorak", 72, 162.5),
            Self::ZamorakBrew => ("Potion of Zamorak", "Torstol", "Janger Berries", 78, 175.0),
        };

        Details::Herblore(HerbloreDetails {
            name: details.0.to_owned(),
            herb: details.1.to_owned(),
            ingredient: details.2.to_owned(),
            level: details.3,
            xp: details.4,
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
pub struct HerbloreDetails {
    pub name: String,
    pub herb: String,
    pub ingredient: String,
    pub level: u32,
    pub xp: f64,
}

impl IntoString for HerbloreDetails {
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
