use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Herblore {
    GuamLeaf,
    AttackPotion,
    Marrentill,
    Antipoison,
    RelicymsBalm,
    Tarromin,
    StrengthPotion,
    Serum207,
    GuthixRest,
    GuamTar,
    Harralander,
    CompostPotion,
    RestorePotion,
    RanarrWeed,
    EnergyPotion,
    Toadflax,
    DefencePotion,
    MarrentillTar,
    AgilityPotion,
    CombatPotion,
    PrayerPotion,
    TarrominTar,
    IritLeaf,
    HarralanderTar,
    SuperAttack,
    Avantoe,
    Superantipoison,
    FishingPotion,
    SuperEnergy,
    HunterPotion,
    Kwuarm,
    GoadingPotion,
    IritTar,
    SuperStrength,
    MagicEssencePotion,
    Huasca,
    PrayerRegenerationPotion,
    Snapdragon,
    WeaponPoison,
    Alcoaugmentator,
    LiplackLiquor,
    MammothmightMix,
    SuperRestore,
    MysticManaAmalgam,
    Cadantine,
    SanfewSerum,
    SuperDefence,
    MarleysMoonlight,
    Lantadyme,
    AntidotePlus,
    AntifirePotion,
    AzureAuraMix,
    DivineSuperAttackPotion,
    DivineSuperDefencePotion,
    DivineSuperStrengthPotion,
    DwarfWeed,
    RangingPotion,
    AqualuxAmalgam,
    WeaponPoisonPlus,
    DivineRangingPotion,
    Torstol,
    MegaliteLiquid,
    MagicPotion,
    StaminaPotion3,
    StaminaPotion4,
    DivineMagicPotion,
    ZamorakBrew,
    AntileechLotion,
    AntidotePlusPlus,
    BastionPotion,
    BattlemagePotion,
    SaradominBrew,
    Mixalot,
    WeaponPoisonPlusPlus,
    ExtendedAntifire3,
    ExtendedAntifire4,
    AncientBrew,
    DivineBastionPotion,
    DivineBattlemagePotion,
    Antivenom3,
    Antivenom4,
    MenaphiteRemedy,
    SuperCombatPotion,
    ForgottenBrew,
    SuperAntifire,
    ExtendedAntivenomPlus,
    AntivenomPlus,
    DivineSuperCombatPotion,
    ExtendedSuperAntifire3,
    ExtendedSuperAntifire4,
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
            Self::Marrentill,
            Self::Antipoison,
            Self::RelicymsBalm,
            Self::Tarromin,
            Self::StrengthPotion,
            Self::Serum207,
            Self::GuthixRest,
            Self::GuamTar,
            Self::Harralander,
            Self::CompostPotion,
            Self::RestorePotion,
            Self::RanarrWeed,
            Self::EnergyPotion,
            Self::Toadflax,
            Self::DefencePotion,
            Self::MarrentillTar,
            Self::AgilityPotion,
            Self::CombatPotion,
            Self::PrayerPotion,
            Self::TarrominTar,
            Self::IritLeaf,
            Self::HarralanderTar,
            Self::SuperAttack,
            Self::Avantoe,
            Self::Superantipoison,
            Self::FishingPotion,
            Self::SuperEnergy,
            Self::HunterPotion,
            Self::Kwuarm,
            Self::GoadingPotion,
            Self::IritTar,
            Self::SuperStrength,
            Self::MagicEssencePotion,
            Self::Huasca,
            Self::PrayerRegenerationPotion,
            Self::Snapdragon,
            Self::WeaponPoison,
            Self::Alcoaugmentator,
            Self::LiplackLiquor,
            Self::MammothmightMix,
            Self::SuperRestore,
            Self::MysticManaAmalgam,
            Self::Cadantine,
            Self::SanfewSerum,
            Self::SuperDefence,
            Self::MarleysMoonlight,
            Self::Lantadyme,
            Self::AntidotePlus,
            Self::AntifirePotion,
            Self::AzureAuraMix,
            Self::DivineSuperAttackPotion,
            Self::DivineSuperDefencePotion,
            Self::DivineSuperStrengthPotion,
            Self::DwarfWeed,
            Self::RangingPotion,
            Self::AqualuxAmalgam,
            Self::WeaponPoisonPlus,
            Self::DivineRangingPotion,
            Self::Torstol,
            Self::MegaliteLiquid,
            Self::MagicPotion,
            Self::StaminaPotion3,
            Self::StaminaPotion4,
            Self::DivineMagicPotion,
            Self::ZamorakBrew,
            Self::AntileechLotion,
            Self::AntidotePlusPlus,
            Self::BastionPotion,
            Self::BattlemagePotion,
            Self::SaradominBrew,
            Self::Mixalot,
            Self::WeaponPoisonPlusPlus,
            Self::ExtendedAntifire3,
            Self::ExtendedAntifire4,
            Self::AncientBrew,
            Self::DivineBastionPotion,
            Self::DivineBattlemagePotion,
            Self::Antivenom3,
            Self::Antivenom4,
            Self::MenaphiteRemedy,
            Self::SuperCombatPotion,
            Self::ForgottenBrew,
            Self::SuperAntifire,
            Self::ExtendedAntivenomPlus,
            Self::AntivenomPlus,
            Self::DivineSuperCombatPotion,
            Self::ExtendedSuperAntifire3,
            Self::ExtendedSuperAntifire4,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::StrengthPotion,
            Self::EnergyPotion,
            Self::PrayerPotion,
            Self::SuperAttack,
            Self::SuperStrength,
            Self::SuperRestore,
            Self::SuperDefence,
            Self::AntifirePotion,
            Self::RangingPotion,
            Self::SaradominBrew,
            Self::SuperCombatPotion,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::GuamLeaf => ("Guam Leaf", 3, 2.5),
            Self::AttackPotion => ("Attack", 3, 25.0),
            Self::Marrentill => ("Marrentill", 5, 3.8),
            Self::Antipoison => ("Antipoison", 5, 37.5),
            Self::RelicymsBalm => ("Relicyms Balm", 8, 40.0),
            Self::Tarromin => ("Tarromin", 11, 5.0),
            Self::StrengthPotion => ("Strength", 12, 50.0),
            Self::Serum207 => ("Serum 207", 15, 50.0),
            Self::GuthixRest => ("Guthix Rest", 18, 59.0),
            Self::GuamTar => ("Guam Tar", 19, 30.0),
            Self::Harralander => ("Harralander", 20, 6.3),
            Self::CompostPotion => ("Compost", 22, 60.0),
            Self::RestorePotion => ("Restore", 22, 62.5),
            Self::RanarrWeed => ("Ranarr Weed", 25, 7.5),
            Self::EnergyPotion => ("Energy", 26, 67.5),
            Self::Toadflax => ("Toadflax", 30, 8.0),
            Self::DefencePotion => ("Defence", 30, 75.0),
            Self::MarrentillTar => ("Marrentill Tar", 31, 42.5),
            Self::AgilityPotion => ("Agility", 34, 80.0),
            Self::CombatPotion => ("Combat", 36, 84.0),
            Self::PrayerPotion => ("Prayer", 38, 87.5),
            Self::TarrominTar => ("Tarromin Tar", 39, 55.0),
            Self::IritLeaf => ("Irit Leaf", 40, 8.8),
            Self::HarralanderTar => ("Harralander Tar", 44, 72.5),
            Self::SuperAttack => ("Super Attack", 45, 100.0),
            Self::Avantoe => ("Avantoe", 48, 10.0),
            Self::Superantipoison => ("Superantipoison", 48, 106.3),
            Self::FishingPotion => ("Fishing", 50, 112.5),
            Self::SuperEnergy => ("Super Energy", 52, 117.5),
            Self::HunterPotion => ("Hunter Potion", 53, 120.0),
            Self::Kwuarm => ("Kwuarm", 54, 11.3),
            Self::GoadingPotion => ("Goading", 54, 132.0),
            Self::IritTar => ("Irit Tar", 55, 85.0),
            Self::SuperStrength => ("Super Strength", 55, 125.0),
            Self::MagicEssencePotion => ("Magic Essence", 57, 130.0),
            Self::Huasca => ("Huasca", 58, 11.8),
            Self::PrayerRegenerationPotion => ("Prayer Regen", 58, 132.0),
            Self::Snapdragon => ("Snapdragon", 59, 11.8),
            Self::WeaponPoison => ("Weapon Poison", 60, 137.5),
            Self::Alcoaugmentator => ("Alcoaugmentator", 60, 190.0),
            Self::LiplackLiquor => ("Liplack Liquor", 60, 190.0),
            Self::MammothmightMix => ("Mammothmight Mix", 60, 190.0),
            Self::SuperRestore => ("Super Restore", 63, 142.5),
            Self::MysticManaAmalgam => ("Mystic Mana Amalgam", 63, 215.0),
            Self::Cadantine => ("Cadantine", 65, 12.5),
            Self::SanfewSerum => ("Sanfew Serum", 65, 160.0),
            Self::SuperDefence => ("Super Defence", 66, 150.0),
            Self::MarleysMoonlight => ("Marleys Moonlight", 66, 240.0),
            Self::Lantadyme => ("Lantadyme", 67, 13.1),
            Self::AntidotePlus => ("Antidote+", 68, 155.0),
            Self::AntifirePotion => ("Antifire Potion", 69, 157.5),
            Self::AzureAuraMix => ("Azure Aura Mix", 69, 265.0),
            Self::DivineSuperAttackPotion => ("Divine Super Attack", 70, 2.0),
            Self::DivineSuperDefencePotion => ("Divine Super Defence", 70, 2.0),
            Self::DivineSuperStrengthPotion => ("Divine Super Strength", 70, 2.0),
            Self::DwarfWeed => ("Dwarf Weed", 70, 13.8),
            Self::RangingPotion => ("Ranging Potion", 72, 162.5),
            Self::AqualuxAmalgam => ("Aqualux Amalgam", 72, 290.0),
            Self::WeaponPoisonPlus => ("Weapon Poison+", 73, 190.0),
            Self::DivineRangingPotion => ("Divine Ranging", 74, 2.0),
            Self::Torstol => ("Torstol", 75, 15.0),
            Self::MegaliteLiquid => ("Megalite Liquid", 75, 315.0),
            Self::MagicPotion => ("Magic Potion", 76, 172.5),
            Self::StaminaPotion3 => ("Stamina3", 77, 76.5),
            Self::StaminaPotion4 => ("Stamina4", 77, 102.0),
            Self::DivineMagicPotion => ("Divine Magic", 78, 2.0),
            Self::ZamorakBrew => ("Zamorak Brew", 78, 175.0),
            Self::AntileechLotion => ("Antileech Lotion", 78, 340.0),
            Self::AntidotePlusPlus => ("Antidote++", 79, 177.5),
            Self::BastionPotion => ("Bastion Potion", 80, 155.0),
            Self::BattlemagePotion => ("Battlemage Potion", 80, 155.0),
            Self::SaradominBrew => ("Saradomin Brew", 81, 180.0),
            Self::Mixalot => ("Mixalot", 81, 365.0),
            Self::WeaponPoisonPlusPlus => ("Weapon Poison++", 82, 190.0),
            Self::ExtendedAntifire3 => ("Ext Antifire3", 84, 82.5),
            Self::ExtendedAntifire4 => ("Ext Antifire4", 84, 110.0),
            Self::AncientBrew => ("Ancient Brew", 85, 190.0),
            Self::DivineBastionPotion => ("Divine Bastion", 86, 2.0),
            Self::DivineBattlemagePotion => ("Divine Battlemage", 86, 2.0),
            Self::Antivenom3 => ("Antivenom3", 87, 90.0),
            Self::Antivenom4 => ("Antivenom4", 87, 120.0),
            Self::MenaphiteRemedy => ("Menaphite Remedy", 88, 200.0),
            Self::SuperCombatPotion => ("Super Combat", 90, 150.0),
            Self::ForgottenBrew => ("Forgotten Brew", 91, 145.0),
            Self::SuperAntifire => ("Super Antifire", 92, 130.0),
            Self::ExtendedAntivenomPlus => ("Ext Antivenom+", 94, 80.0),
            Self::AntivenomPlus => ("Antivenom+", 94, 125.0),
            Self::DivineSuperCombatPotion => ("Divine Super Combat", 97, 2.0),
            Self::ExtendedSuperAntifire3 => ("Ext Super Antifire3", 98, 120.0),
            Self::ExtendedSuperAntifire4 => ("Ext Super Antifire4", 98, 160.0),
        };

        Details::Herblore(HerbloreDetails {
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
pub struct HerbloreDetails {
    pub name: String,
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
