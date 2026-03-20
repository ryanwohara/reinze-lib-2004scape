use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Thieving {
    ManOrWoman,
    WinterSqirkjuice,
    VegetableStall,
    CakeStall,
    MonkeyFoodStall,
    TeaStall,
    CraftingStall,
    MonkeyGeneralStall,
    Farmer,
    Chest10Coins,
    HamMember,
    SilkStall,
    WineStall,
    WarriorWomenOrAlKharidWarrior,
    FruitStall,
    SpringSqirkjuice,
    SeedStall,
    NatureRuneChest,
    IsleOfSoulsChest,
    Rogue,
    FurStall,
    CaveGoblin,
    MasterFarmer,
    Guard,
    FishStall,
    Chest50Coins,
    BeardedPollnivnianBandit,
    FremennikCitizen,
    AutumnSqirkjuice,
    ChestSteelArrowtips,
    CrossbowStall,
    WallSafe,
    WealthyCitizen,
    SilverStall,
    DorgeshKaanAverageChest,
    DesertBandit,
    Knight,
    PollnivnianBandit,
    StoneChest,
    MagicStall,
    SpiceStall,
    MenaphiteThug,
    YanilleWatchman,
    ScimitarStall,
    SummerSqirkjuice,
    Paladin,
    Gnome,
    GemStall,
    DorgeshKaanRichChest,
    Hero,
    Vyre,
    OreStall,
    RoguesCastleChest,
    Elf,
    TzhaarHur,
}

impl Skill for Thieving {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::ManOrWoman,
            Self::WinterSqirkjuice,
            Self::VegetableStall,
            Self::CakeStall,
            Self::MonkeyFoodStall,
            Self::TeaStall,
            Self::CraftingStall,
            Self::MonkeyGeneralStall,
            Self::Farmer,
            Self::Chest10Coins,
            Self::HamMember,
            Self::SilkStall,
            Self::WineStall,
            Self::WarriorWomenOrAlKharidWarrior,
            Self::FruitStall,
            Self::SpringSqirkjuice,
            Self::SeedStall,
            Self::NatureRuneChest,
            Self::IsleOfSoulsChest,
            Self::Rogue,
            Self::FurStall,
            Self::CaveGoblin,
            Self::MasterFarmer,
            Self::Guard,
            Self::FishStall,
            Self::Chest50Coins,
            Self::BeardedPollnivnianBandit,
            Self::FremennikCitizen,
            Self::AutumnSqirkjuice,
            Self::ChestSteelArrowtips,
            Self::CrossbowStall,
            Self::WallSafe,
            Self::WealthyCitizen,
            Self::SilverStall,
            Self::DorgeshKaanAverageChest,
            Self::DesertBandit,
            Self::Knight,
            Self::PollnivnianBandit,
            Self::StoneChest,
            Self::MagicStall,
            Self::SpiceStall,
            Self::MenaphiteThug,
            Self::YanilleWatchman,
            Self::ScimitarStall,
            Self::SummerSqirkjuice,
            Self::Paladin,
            Self::Gnome,
            Self::GemStall,
            Self::DorgeshKaanRichChest,
            Self::Hero,
            Self::Vyre,
            Self::OreStall,
            Self::RoguesCastleChest,
            Self::Elf,
            Self::TzhaarHur,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::MasterFarmer,
            Self::Guard,
            Self::Knight,
            Self::SummerSqirkjuice,
            Self::Vyre,
            Self::Elf,
            Self::TzhaarHur,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::ManOrWoman => ("Man / Woman", 1, 8.0),
            Self::WinterSqirkjuice => ("Winter Sq'irkjuice", 1, 350.0),
            Self::VegetableStall => ("Vegetable Stall", 2, 10.0),
            Self::CakeStall => ("Cake Stall", 5, 16.0),
            Self::MonkeyFoodStall => ("Monkey Food Stall", 5, 16.0),
            Self::TeaStall => ("Tea Stall", 5, 16.0),
            Self::CraftingStall => ("Crafting Stall", 5, 20.0),
            Self::MonkeyGeneralStall => ("Monkey General Stall", 5, 25.0),
            Self::Farmer => ("Farmer", 10, 14.5),
            Self::Chest10Coins => ("Chest (10 Coins)", 13, 7.8),
            Self::HamMember => ("H.A.M. Member", 15, 22.2),
            Self::SilkStall => ("Silk Stall", 20, 24.0),
            Self::WineStall => ("Wine Stall", 22, 27.0),
            Self::WarriorWomenOrAlKharidWarrior => ("Warrior Women / Al-Kharid Warrior", 25, 26.0),
            Self::FruitStall => ("Fruit Stall", 25, 28.5),
            Self::SpringSqirkjuice => ("Spring Sq'irkjuice", 25, 1350.0),
            Self::SeedStall => ("Seed Stall", 27, 10.0),
            Self::NatureRuneChest => ("Nature Rune Chest", 28, 25.0),
            Self::IsleOfSoulsChest => ("Isle of Souls Dungeon Chest", 28, 150.0),
            Self::Rogue => ("Rogue", 32, 36.5),
            Self::FurStall => ("Fur Stall", 35, 45.0),
            Self::CaveGoblin => ("Cave Goblin", 36, 40.0),
            Self::MasterFarmer => ("Master Farmer", 38, 43.0),
            Self::Guard => ("Guard", 40, 46.8),
            Self::FishStall => ("Fish Stall", 42, 42.0),
            Self::Chest50Coins => ("Chest (50 Coins)", 43, 125.0),
            Self::BeardedPollnivnianBandit => ("Bearded Pollnivnian Bandit", 45, 65.0),
            Self::FremennikCitizen => ("Fremennik Citizen", 45, 65.0),
            Self::AutumnSqirkjuice => ("Autumn Sq'irkjuice", 45, 2350.0),
            Self::ChestSteelArrowtips => ("Chest (Steel Arrowtips)", 47, 150.0),
            Self::CrossbowStall => ("Crossbow Stall", 49, 52.0),
            Self::WallSafe => ("Wall Safe", 50, 70.0),
            Self::WealthyCitizen => ("Wealthy Citizen", 50, 96.0),
            Self::SilverStall => ("Silver Stall", 50, 205.0),
            Self::DorgeshKaanAverageChest => ("Dorgesh-Kaan Average Chest", 52, 200.0),
            Self::DesertBandit => ("Desert Bandit", 53, 79.4),
            Self::Knight => ("Knight", 55, 84.3),
            Self::PollnivnianBandit => ("Pollnivnian Bandit", 55, 84.3),
            Self::StoneChest => ("Stone Chest", 64, 280.0),
            Self::MagicStall => ("Magic Stall", 65, 90.0),
            Self::SpiceStall => ("Spice Stall", 65, 92.0),
            Self::MenaphiteThug => ("Menaphite Thug", 65, 137.5),
            Self::YanilleWatchman => ("Yanille Watchman", 65, 137.5),
            Self::ScimitarStall => ("Scimitar Stall", 65, 210.0),
            Self::SummerSqirkjuice => ("Summer Sq'irkjuice", 65, 3000.0),
            Self::Paladin => ("Paladin", 70, 131.8),
            Self::Gnome => ("Gnome", 75, 133.5),
            Self::GemStall => ("Gem Stall", 75, 408.0),
            Self::DorgeshKaanRichChest => ("Dorgesh-Kaan Rich Chest", 78, 650.0),
            Self::Hero => ("Hero", 80, 163.3),
            Self::Vyre => ("Vyre", 82, 306.9),
            Self::OreStall => ("Ore Stall", 82, 350.0),
            Self::RoguesCastleChest => ("Wilderness Rogues' Chest", 84, 701.7),
            Self::Elf => ("Elf", 85, 353.3),
            Self::TzhaarHur => ("TzHaar-Hur", 90, 103.4),
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
