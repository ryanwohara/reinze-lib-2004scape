use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Magic {
    WindStrike,
    Confuse,
    WaterStrike,
    EnchantLvl1Jewelry,
    EarthStrike,
    Weaken,
    FireStrike,
    BonesToBananas,
    WindBolt,
    Curse,
    Bind,
    LowLevelAlchemy,
    WaterBolt,
    VarrockTeleport,
    EnchantLvl2Jewelry,
    EarthBolt,
    LumbridgeTeleport,
    TelekineticGrab,
    FireBolt,
    FaladorTeleport,
    CrumbleUndead,
    WindBlast,
    SuperheatItem,
    CamelotTeleport,
    WaterBlast,
    EnchantLvl3Jewelry,
    IbanBlast,
    Snare,
    ArdougneTeleport,
    EarthBlast,
    HighLevelAlchemy,
    ChargeWaterOrb,
    EnchantLvl4Jewelry,
    WatchtowerTeleport,
    FireBlast,
    ChargeEarthOrb,
    SaradominStrike,
    ClawsOfGuthix,
    FlamesOfZamorak,
    WindWave,
    ChargeFireOrb,
    WaterWave,
    ChargeAirOrb,
    Vulnerability,
    EnchantLvl5Jewelry,
    EarthWave,
    Enfeeble,
    FireWave,
    Entangle,
    Stun,
    Charge,
}

impl Detail for Magic {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Magic(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Magic(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Magic(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Magic {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::WindStrike,
            Self::Confuse,
            Self::WaterStrike,
            Self::EnchantLvl1Jewelry,
            Self::EarthStrike,
            Self::Weaken,
            Self::FireStrike,
            Self::BonesToBananas,
            Self::WindBolt,
            Self::Curse,
            Self::Bind,
            Self::LowLevelAlchemy,
            Self::WaterBolt,
            Self::VarrockTeleport,
            Self::EnchantLvl2Jewelry,
            Self::EarthBolt,
            Self::LumbridgeTeleport,
            Self::TelekineticGrab,
            Self::FireBolt,
            Self::FaladorTeleport,
            Self::CrumbleUndead,
            Self::WindBlast,
            Self::SuperheatItem,
            Self::CamelotTeleport,
            Self::WaterBlast,
            Self::EnchantLvl3Jewelry,
            Self::IbanBlast,
            Self::Snare,
            Self::ArdougneTeleport,
            Self::EarthBlast,
            Self::HighLevelAlchemy,
            Self::ChargeWaterOrb,
            Self::EnchantLvl4Jewelry,
            Self::WatchtowerTeleport,
            Self::FireBlast,
            Self::ChargeEarthOrb,
            Self::SaradominStrike,
            Self::ClawsOfGuthix,
            Self::FlamesOfZamorak,
            Self::WindWave,
            Self::ChargeFireOrb,
            Self::WaterWave,
            Self::ChargeAirOrb,
            Self::Vulnerability,
            Self::EnchantLvl5Jewelry,
            Self::EarthWave,
            Self::Enfeeble,
            Self::FireWave,
            Self::Entangle,
            Self::Stun,
            Self::Charge,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::VarrockTeleport,
            Self::CamelotTeleport,
            Self::ArdougneTeleport,
            Self::HighLevelAlchemy,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::WindStrike => (
                "Wind Strike",
                1,
                5.5,
                false,
                "1 Air, 1 Mind",
                "A basic Air missile",
            ),
            Self::Confuse => (
                "Confuse",
                3,
                13.0,
                false,
                "3 Water, 2 Earth, 1 Body",
                "Reduces your opponent's attack by 5%",
            ),
            Self::WaterStrike => (
                "Water Strike",
                3,
                7.5,
                false,
                "1 Water, 1 Air, 1 Mind",
                "A basic Water missile",
            ),
            Self::EnchantLvl1Jewelry => (
                "Enchant Lvl-1 Jewelry",
                7,
                17.5,
                false,
                "1 Water, 1 Cosmic",
                "For use on Sapphire Jewelry",
            ),
            Self::EarthStrike => (
                "Earth Strike",
                9,
                9.5,
                false,
                "2 Earth, 1 Air, 1 Mind",
                "A basic Earth missile",
            ),
            Self::Weaken => (
                "Weaken",
                11,
                21.0,
                false,
                "3 Water, 2 Earth, 1 Body",
                "Reduces your opponent's strength by 5%",
            ),
            Self::FireStrike => (
                "Fire Strike",
                13,
                11.5,
                false,
                "3 Fire, 2 Air, 1 Mind",
                "A basic Fire missile",
            ),
            Self::BonesToBananas => (
                "Bones to Bananas",
                15,
                25.0,
                false,
                "2 Earth, 2 Water, 1 Nature",
                "Changes all held bones into bananas",
            ),
            Self::WindBolt => (
                "Wind Bolt",
                17,
                13.5,
                false,
                "2 Air, 1 Chaos",
                "A low level Air missile",
            ),
            Self::Curse => (
                "Curse",
                19,
                29.0,
                false,
                "2 Water, 3 Earth, 1 Body",
                "Reduces your opponent's defense by 5%",
            ),
            Self::Bind => (
                "Bind",
                20,
                30.0,
                false,
                "3 Earth, 3 Water, 2 Nature",
                "Holds your opponent for 5 seconds",
            ),
            Self::LowLevelAlchemy => (
                "Low Level Alchemy",
                21,
                31.0,
                false,
                "3 Fire, 1 Nature",
                "Converts an item into gold",
            ),
            Self::WaterBolt => (
                "Water Bolt",
                23,
                16.5,
                false,
                "2 Water, 2 Air, 1 Chaos",
                "A low level Water missile",
            ),
            Self::VarrockTeleport => (
                "Varrock Teleport",
                25,
                35.0,
                false,
                "1 Fire, 3 Air, 1 Law",
                "Teleports you to Varrock",
            ),
            Self::EnchantLvl2Jewelry => (
                "Enchant Lvl-2 Jewelry",
                27,
                37.0,
                false,
                "3 Air, 1 Cosmic",
                "For use on Emerald Jewelry",
            ),
            Self::EarthBolt => (
                "Earth Bolt",
                29,
                19.5,
                false,
                "3 Earth, 2 Water, 1 Chaos",
                "A low level Earth missile",
            ),
            Self::LumbridgeTeleport => (
                "Lumbridge Teleport",
                31,
                41.0,
                false,
                "1 Earth, 3 Air, 1 Law",
                "Teleports you to Lumbridge",
            ),
            Self::TelekineticGrab => (
                "Telekinetic Grab",
                33,
                43.0,
                false,
                "1 Air, 1 Law",
                "Take an item you can see but can't reach",
            ),
            Self::FireBolt => (
                "Fire Bolt",
                35,
                22.5,
                false,
                "4 Fire, 3 Air, 1 Chaos",
                "A low level Fire missile",
            ),
            Self::FaladorTeleport => (
                "Falador Teleport",
                37,
                48.0,
                false,
                "1 Water, 3 Air, 1 Law",
                "Teleports you to Falador",
            ),
            Self::CrumbleUndead => (
                "Crumble Undead",
                39,
                24.5,
                false,
                "2 Earth, 2 Air, 1 Chaos",
                "Hits skeletons, ghosts, and zombies hard",
            ),
            Self::WindBlast => (
                "Wind Blast",
                41,
                25.5,
                false,
                "3 Air, 1 Death",
                "A medium level Air missile",
            ),
            Self::SuperheatItem => (
                "Superheat Item",
                43,
                53.0,
                false,
                "4 Fire, 1 Nature",
                "Smelt ore without a furnace",
            ),
            Self::CamelotTeleport => (
                "Camelot Teleport",
                45,
                55.5,
                true,
                "5 Air, 1 Law",
                "Teleports you to Camelot",
            ),
            Self::WaterBlast => (
                "Water Blast",
                47,
                28.5,
                false,
                "3 Water, 3 Air, 1 Death",
                "A medium level Water missile",
            ),
            Self::EnchantLvl3Jewelry => (
                "Enchant Lvl-3 Jewelry",
                49,
                59.0,
                false,
                "5 Fire, 1 Cosmic",
                "For use on Ruby Jewelry",
            ),
            Self::IbanBlast => (
                "Iban Blast",
                50,
                30.0,
                true,
                "5 Fire, 1 Death, Staff of Iban",
                "A strength 25 missile attack",
            ),
            Self::Snare => (
                "Snare",
                50,
                60.0,
                true,
                "4 Earth, 4 Water, 3 Nature",
                "Holds your opponent for 10 seconds",
            ),
            Self::ArdougneTeleport => (
                "Ardougne Teleport",
                51,
                61.0,
                true,
                "2 Water, 2 Law",
                "Teleports you to Ardougne",
            ),
            Self::EarthBlast => (
                "Earth Blast",
                53,
                31.5,
                false,
                "4 Earth, 3 Air, 1 Death",
                "A medium level Earth missile",
            ),
            Self::HighLevelAlchemy => (
                "High Level Alchemy",
                55,
                65.0,
                false,
                "5 Fire, 1 Nature",
                "Converts an item into gold",
            ),
            Self::ChargeWaterOrb => (
                "Charge Water Orb",
                56,
                66.0,
                true,
                "30 Water, 3 Cosmic, Orb",
                "Needs to be cast on a water obelisk",
            ),
            Self::EnchantLvl4Jewelry => (
                "Enchant Lvl-4 Jewelry",
                57,
                67.0,
                false,
                "10 Earth, 1 Cosmic",
                "For use on Diamond Jewelry",
            ),
            Self::WatchtowerTeleport => (
                "Watchtower Teleport",
                58,
                68.0,
                true,
                "2 Earth, 2 Law",
                "Teleports you to the Watchtower",
            ),
            Self::FireBlast => (
                "Fire Blast",
                59,
                34.5,
                false,
                "5 Fire, 4 Air, 1 Death",
                "A medium level Fire missile",
            ),
            Self::ChargeEarthOrb => (
                "Charge Earth Orb",
                60,
                70.0,
                true,
                "30 Earth, 3 Cosmic, Orb",
                "Needs to be cast on an earth obelisk",
            ),
            Self::SaradominStrike => (
                "Saradomin Strike",
                60,
                35.0,
                true,
                "2 Fire, 2 Blood, 4 Air, Staff of Saradomin",
                "Summons the power of Saradomin",
            ),
            Self::ClawsOfGuthix => (
                "Claws of Guthix",
                60,
                35.0,
                true,
                "1 Fire, 2 Blood, 4 Air, Staff of Guthix",
                "Summons the power of Guthix",
            ),
            Self::FlamesOfZamorak => (
                "Flames of Zamorak",
                60,
                35.0,
                true,
                "4 Fire, 2 Blood, 1 Air, Staff of Zamorak",
                "Summons the power of Zamorak",
            ),
            Self::WindWave => (
                "Wind Wave",
                62,
                36.0,
                true,
                "5 Air, 1 Blood",
                "A high level Air missile",
            ),
            Self::ChargeFireOrb => (
                "Charge Fire Orb",
                63,
                73.0,
                true,
                "30 Fire, 3 Cosmic, Orb",
                "Needs to be cast on a fire obelisk",
            ),
            Self::WaterWave => (
                "Water Wave",
                65,
                37.5,
                true,
                "7 Water, 5 Air, 1 Blood",
                "A high level Water missile",
            ),
            Self::ChargeAirOrb => (
                "Charge Air Orb",
                66,
                76.0,
                true,
                "30 Air, 3 Cosmic, Orb",
                "Needs to be cast on an air obelisk",
            ),
            Self::Vulnerability => (
                "Vulnerability",
                66,
                76.0,
                true,
                "5 Earth, 5 Water, 1 Soul",
                "Reduces your opponent's defense by 10%",
            ),
            Self::EnchantLvl5Jewelry => (
                "Enchant Lvl-5 Jewelry",
                69,
                78.0,
                true,
                "15 Water, 15 Earth, 1 Cosmic",
                "For use on Dragonstone Jewelry",
            ),
            Self::EarthWave => (
                "Earth Wave",
                70,
                40.0,
                true,
                "7 Earth, 5 Air, 1 Blood",
                "A high level Earth missile",
            ),
            Self::Enfeeble => (
                "Enfeeble",
                73,
                83.0,
                true,
                "8 Earth, 8 Water, 1 Soul",
                "Reduces your opponent's strength by 10%",
            ),
            Self::FireWave => (
                "Fire Wave",
                75,
                42.5,
                true,
                "7 Fire, 5 Air, 1 Blood",
                "A high level Fire missile",
            ),
            Self::Entangle => (
                "Entangle",
                79,
                89.0,
                true,
                "5 Earth, 5 Water, 4 Nature",
                "Holds your opponent for 15 seconds",
            ),
            Self::Stun => (
                "Stun",
                80,
                90.0,
                true,
                "12 Earth, 12 Water, 1 Soul",
                "Reduces your opponent's attack by 10%",
            ),
            Self::Charge => (
                "Charge",
                80,
                180.0,
                true,
                "3 Fire, 3 Blood, 3 Air",
                "Temporarily increases the power of the three arena spells",
            ),
        };

        Details::Magic(MagicDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            members: details.3,
            runes: details.4.to_owned(),
            effect: details.5.to_owned(),
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
pub struct MagicDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub members: bool,
    pub runes: String,
    pub effect: String,
}

impl IntoString for MagicDetails {
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
