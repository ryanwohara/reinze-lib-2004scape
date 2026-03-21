use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Thieving {
    Man,
    Farmer,
    Warrior,
    Rogue,
    Guard,
    Knight,
    YanilleWatchman,
    Paladin,
    Gnome,
    Hero,
    TeaStall,
    BakersStall,
    SilkStall,
    FurStall,
    SilverStall,
    SpiceStall,
    GemStall,
    ChestCoins10,
    ChestNatureRunes,
    ChestCoins50,
    ChestSteelArrowheads,
    ChestBloodRunes,
    PaladinChest,
    DoorSpiceStall,
    DoorHandelmortMansion,
    DoorRedCarpetHouse,
    DoorBakersStallHouse,
    DoorYanilleBank,
    DoorArdougneSewers,
    DoorChaosDruidTower,
    DoorIbansCave,
    DoorPaladinTower,
    DoorAgilityDungeon,
    DoorPirate,
}

impl Skill for Thieving {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Man,
            Self::DoorSpiceStall,
            Self::TeaStall,
            Self::BakersStall,
            Self::ChestCoins10,
            Self::Farmer,
            Self::DoorBakersStallHouse,
            Self::DoorRedCarpetHouse,
            Self::DoorYanilleBank,
            Self::SilkStall,
            Self::DoorHandelmortMansion,
            Self::Warrior,
            Self::ChestNatureRunes,
            Self::Rogue,
            Self::DoorArdougneSewers,
            Self::FurStall,
            Self::DoorPirate,
            Self::Guard,
            Self::ChestCoins50,
            Self::DoorChaosDruidTower,
            Self::ChestSteelArrowheads,
            Self::DoorIbansCave,
            Self::SilverStall,
            Self::Knight,
            Self::ChestBloodRunes,
            Self::DoorPaladinTower,
            Self::SpiceStall,
            Self::YanilleWatchman,
            Self::Paladin,
            Self::PaladinChest,
            Self::Gnome,
            Self::GemStall,
            Self::Hero,
            Self::DoorAgilityDungeon,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::Guard,
            Self::Knight,
            Self::YanilleWatchman,
            Self::Paladin,
            Self::Gnome,
            Self::Hero,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::Man => ("Man", 1, 8.0),
            Self::Farmer => ("Farmer", 10, 14.5),
            Self::Warrior => ("Warrior", 25, 26.0),
            Self::Rogue => ("Rogue", 32, 35.5),
            Self::Guard => ("Guard", 40, 46.5),
            Self::Knight => ("Knight", 55, 84.5),
            Self::YanilleWatchman => ("Yanille Watchman", 65, 137.5),
            Self::Paladin => ("Paladin", 70, 151.75),
            Self::Gnome => ("Gnome", 75, 198.5),
            Self::Hero => ("Hero", 80, 275.0),
            Self::TeaStall => ("Tea Stall", 5, 16.0),
            Self::BakersStall => ("Bakers Stall", 5, 16.0),
            Self::SilkStall => ("Silk Stall", 20, 24.0),
            Self::FurStall => ("Fur Stall", 35, 36.0),
            Self::SilverStall => ("Silver Stall", 50, 54.0),
            Self::SpiceStall => ("Spice Stall", 65, 81.0),
            Self::GemStall => ("Gem Stall", 75, 16.0),
            Self::ChestCoins10 => ("Chest (10 Coins)", 10, 7.0),
            Self::ChestNatureRunes => ("Chest (Nature Runes)", 28, 25.0),
            Self::ChestCoins50 => ("Chest (50 Coins)", 43, 125.0),
            Self::ChestSteelArrowheads => ("Chest (Steel Arrowheads)", 47, 150.0),
            Self::ChestBloodRunes => ("Chest (Blood Runes)", 59, 250.0),
            Self::PaladinChest => ("Paladin Chest", 72, 500.0),
            Self::DoorSpiceStall => ("Door (Spice Stall)", 1, 3.75),
            Self::DoorHandelmortMansion => ("Door (Handelmort Mansion)", 21, 15.0),
            Self::DoorRedCarpetHouse => ("Door (Red Carpet House)", 16, 15.0),
            Self::DoorBakersStallHouse => ("Door (Bakers Stall House)", 14, 15.0),
            Self::DoorYanilleBank => ("Door (Yanille Bank)", 16, 15.0),
            Self::DoorArdougneSewers => ("Door (Ardougne Sewers)", 32, 25.0),
            Self::DoorChaosDruidTower => ("Door (Chaos Druid Tower)", 46, 37.5),
            Self::DoorIbansCave => ("Door (Ibans Cave)", 50, 4.0),
            Self::DoorPaladinTower => ("Door (Paladin Tower)", 61, 50.0),
            Self::DoorAgilityDungeon => ("Door (Agility Dungeon)", 82, 50.0),
            Self::DoorPirate => ("Door (Pirate)", 39, 35.0),
        };

        Details::Thieving(ThievingDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: vec![],
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

impl Detail for Thieving {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Thieving(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Thieving(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Thieving(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct ThievingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for ThievingDetails {
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
