use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Runecraft {
    AirRune,
    MindRune,
    WaterRune,
    EarthRune,
    FireRune,
    BodyRune,
    CosmicRune,
    ChaosRune,
    NatureRune,
    LawRune,
}

impl Skill for Runecraft {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::AirRune,
            Self::MindRune,
            Self::WaterRune,
            Self::EarthRune,
            Self::FireRune,
            Self::BodyRune,
            Self::CosmicRune,
            Self::ChaosRune,
            Self::NatureRune,
            Self::LawRune,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::AirRune,
            Self::FireRune,
            Self::CosmicRune,
            Self::ChaosRune,
            Self::NatureRune,
            Self::LawRune,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::AirRune => ("Air Rune", 1, 5.0),
            Self::MindRune => ("Mind Rune", 2, 5.5),
            Self::WaterRune => ("Water Rune", 5, 6.0),
            Self::EarthRune => ("Earth Rune", 9, 6.5),
            Self::FireRune => ("Fire Rune", 14, 7.0),
            Self::BodyRune => ("Body Rune", 20, 7.5),
            Self::CosmicRune => ("Cosmic Rune", 27, 8.0),
            Self::ChaosRune => ("Chaos Rune", 35, 8.5),
            Self::NatureRune => ("Nature Rune", 44, 9.0),
            Self::LawRune => ("Law Rune", 54, 9.5),
        };

        Details::Runecraft(RunecraftDetails {
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

impl Detail for Runecraft {
    fn multipliers(&self) -> Vec<Multipliers> {
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
        vec![format!(
            "{}: {}",
            s.c1(self.name.as_str()),
            s.c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp as f64).ceil()).as_str(),
                "d"
            )
            .as_str())
        )]
        .join(" ")
    }
}
