use crate::stats::runecraft::RunecraftMultipliers::DaeyaltEssence;
use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Runecraft {
    AirRune,
    AirTiara,
    MindTiara,
    WaterTiara,
    EarthTiara,
    FireTiara,
    BodyTiara,
    CosmicTiara,
    ChaosTiara,
    NatureTiara,
    LawTiara,
    DeathTiara,
    WrathTiara,
    MindRune,
    MindCore,
    WaterRune,
    MistRune,
    EarthRune,
    DustRune,
    MudRune,
    FireRune,
    SmokeRune,
    SteamRune,
    BodyRune,
    BodyCore,
    LavaRune,
    CosmicRune,
    SunfireRune,
    ChaosRune,
    ChaosCore,
    AstralRune,
    NatureRune,
    LawRune,
    DeathRune,
    TrueBloodRune,
    ZeahBloodRune,
    SoulRune,
    WrathRune,
}

impl Skill for Runecraft {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::AirRune,
            Self::AirTiara,
            Self::MindTiara,
            Self::WaterTiara,
            Self::EarthTiara,
            Self::FireTiara,
            Self::BodyTiara,
            Self::CosmicTiara,
            Self::ChaosTiara,
            Self::NatureTiara,
            Self::LawTiara,
            Self::DeathTiara,
            Self::WrathTiara,
            Self::MindRune,
            Self::MindCore,
            Self::WaterRune,
            Self::MistRune,
            Self::EarthRune,
            Self::DustRune,
            Self::MudRune,
            Self::FireRune,
            Self::SmokeRune,
            Self::SteamRune,
            Self::BodyRune,
            Self::BodyCore,
            Self::LavaRune,
            Self::CosmicRune,
            Self::SunfireRune,
            Self::ChaosRune,
            Self::ChaosCore,
            Self::AstralRune,
            Self::NatureRune,
            Self::LawRune,
            Self::DeathRune,
            Self::TrueBloodRune,
            Self::ZeahBloodRune,
            Self::SoulRune,
            Self::WrathRune,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::FireRune,
            Self::CosmicRune,
            Self::ChaosRune,
            Self::AstralRune,
            Self::NatureRune,
            Self::LawRune,
            Self::ZeahBloodRune,
            Self::SoulRune,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::AirRune => ("Air Rune", 1, 5.0, false),
            Self::AirTiara => ("Air Tiara", 1, 25.0, true),
            Self::MindTiara => ("Mind Tiara", 1, 27.5, true),
            Self::WaterTiara => ("Water Tiara", 1, 30.0, true),
            Self::EarthTiara => ("Earth Tiara", 1, 32.5, true),
            Self::FireTiara => ("Fire Tiara", 1, 35.0, true),
            Self::BodyTiara => ("Body Tiara", 1, 37.5, true),
            Self::CosmicTiara => ("Cosmic Tiara", 1, 40.0, true),
            Self::ChaosTiara => ("Chaos Tiara", 1, 42.5, true),
            Self::NatureTiara => ("Nature Tiara", 1, 45.0, true),
            Self::LawTiara => ("Law Tiara", 1, 47.5, true),
            Self::DeathTiara => ("Death Tiara", 1, 50.0, true),
            Self::WrathTiara => ("Wrath Tiara", 1, 52.5, true),
            Self::MindRune => ("Mind Rune", 2, 5.5, false),
            Self::MindCore => ("Mind Core", 2, 55.0, true),
            Self::WaterRune => ("Water Rune", 5, 6.0, false),
            Self::MistRune => ("Mist Rune", 6, 8.5, false),
            Self::EarthRune => ("Earth Rune", 9, 6.5, false),
            Self::DustRune => ("Dust Rune", 10, 9.0, false),
            Self::MudRune => ("Mud Rune", 13, 9.5, false),
            Self::FireRune => ("Fire Rune", 14, 7.0, false),
            Self::SmokeRune => ("Smoke Rune", 15, 9.5, false),
            Self::SteamRune => ("Steam Rune", 19, 10.0, false),
            Self::BodyRune => ("Body Rune", 20, 7.5, false),
            Self::BodyCore => ("Body Core", 20, 75.0, true),
            Self::LavaRune => ("Lava Rune", 23, 10.5, false),
            Self::CosmicRune => ("Cosmic Rune", 27, 8.0, false),
            Self::SunfireRune => ("Sunfire Rune", 33, 9.0, false),
            Self::ChaosRune => ("Chaos Rune", 35, 8.5, false),
            Self::ChaosCore => ("Chaos Core", 35, 85.0, true),
            Self::AstralRune => ("Astral Rune", 40, 8.7, false),
            Self::NatureRune => ("Nature Rune", 44, 9.0, false),
            Self::LawRune => ("Law Rune", 54, 9.5, false),
            Self::DeathRune => ("Death Rune", 65, 10.0, false),
            Self::TrueBloodRune => ("True Blood Rune", 77, 10.5, false),
            Self::ZeahBloodRune => ("Zeah Blood Rune", 77, 24.425, true),
            Self::SoulRune => ("Soul Rune", 90, 30.325, true),
            Self::WrathRune => ("Wrath Rune", 95, 8.0, false),
        };

        Details::Runecraft(RunecraftDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: if !details.3 {
                vec![Multipliers::Runecraft(DaeyaltEssence)]
            } else {
                vec![]
            },
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

impl Detail for Runecraft {
    fn multipliers(&self) -> Vec<Multipliers> {
        if let Details::Runecraft(obj) = self.details() {
            return obj.multipliers;
        }

        vec![]
    }

    fn name(&self) -> String {
        if let Details::Runecraft(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Runecraft(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Runecraft(obj) = self.details() {
            return obj.xp;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct RunecraftDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for RunecraftDetails {
    fn to_string(&self, s: &crate::stats::skill::Source, xp_difference: f64) -> String {
        let mut vec = vec![format!(
            "{}: {}",
            s.c1(self.name.as_str()),
            s.c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp as f64).ceil()).as_str(),
                "d"
            )
            .as_str())
        )];

        self.multipliers.iter().for_each(|x| {
            let a = match x {
                Multipliers::Runecraft(x) => x,
                _ => return,
            };
            let d = a.details();
            vec.push(s.p(format!(
                "{} {}",
                s.c1(format!("{}:", d.name.as_str()).as_str()),
                s.c2(common::commas_from_string(
                    format!("{}", (xp_difference / (self.xp as f64 * d.value)).ceil()).as_str(),
                    "d"
                )
                .as_str())
            )
            .as_str()));
        });

        vec.join(" ")
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum RunecraftMultipliers {
    DaeyaltEssence,
}

impl RunecraftMultipliers {
    pub fn details(&self) -> RunecraftMultiplierDetails {
        let details = match self {
            Self::DaeyaltEssence => ("Daeyalt", 1.025),
        };

        RunecraftMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct RunecraftMultiplierDetails {
    pub name: String,
    pub value: f64,
}
