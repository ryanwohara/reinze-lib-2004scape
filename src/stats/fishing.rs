use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Fishing {
    Shrimps,
    Karambwanji,
    Sardine,
    Guppy,
    Herring,
    Anchovies,
    Mackerel,
    Cavefish,
    Bream,
    Trout,
    Cod,
    Pike,
    SlimyEel,
    Salmon,
    Tetra,
    Tuna,
    CaveEel,
    RainbowFish,
    Lobster,
    Catfish,
    Bass,
    LeapingTrout,
    Swordfish,
    LeapingSalmon,
    Monkfish,
    Karambwan,
    LeapingSturgeon,
    Shark,
    SeaTurtle,
    InfernalEel,
    MantaRay,
    Minnow,
    Anglerfish,
    DarkCrab,
    SacredEel,
}

impl Detail for Fishing {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Fishing(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Fishing(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Fishing(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Fishing {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Shrimps,
            Self::Karambwanji,
            Self::Sardine,
            Self::Guppy,
            Self::Herring,
            Self::Anchovies,
            Self::Mackerel,
            Self::Cavefish,
            Self::Bream,
            Self::Trout,
            Self::Cod,
            Self::Pike,
            Self::SlimyEel,
            Self::Salmon,
            Self::Tetra,
            Self::Tuna,
            Self::CaveEel,
            Self::RainbowFish,
            Self::Lobster,
            Self::Catfish,
            Self::Bass,
            Self::LeapingTrout,
            Self::Swordfish,
            Self::LeapingSalmon,
            Self::Monkfish,
            Self::Karambwan,
            Self::LeapingSturgeon,
            Self::Shark,
            Self::SeaTurtle,
            Self::InfernalEel,
            Self::MantaRay,
            Self::Minnow,
            Self::Anglerfish,
            Self::DarkCrab,
            Self::SacredEel,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::Trout,
            Self::Salmon,
            Self::Tuna,
            Self::Lobster,
            Self::Swordfish,
            Self::Monkfish,
            Self::Shark,
            Self::Minnow,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::Shrimps => ("Shrimps", 1, 10.0),
            Self::Karambwanji => ("Karambwanji", 5, 5.0),
            Self::Sardine => ("Sardine", 5, 20.0),
            Self::Guppy => ("Guppy", 7, 8.0),
            Self::Herring => ("Herring", 10, 30.0),
            Self::Anchovies => ("Anchovies", 15, 40.0),
            Self::Mackerel => ("Mackerel", 16, 20.0),
            Self::Cavefish => ("Cavefish", 20, 16.0),
            Self::Bream => ("Bream", 20, 20.0),
            Self::Trout => ("Trout", 20, 50.0),
            Self::Cod => ("Cod", 23, 45.0),
            Self::Pike => ("Pike", 25, 60.0),
            Self::SlimyEel => ("Slimy_Eel", 28, 80.0),
            Self::Salmon => ("Salmon", 30, 70.0),
            Self::Tetra => ("Tetra", 33, 24.0),
            Self::Tuna => ("Tuna", 35, 80.0),
            Self::CaveEel => ("Cave_Eel", 38, 80.0),
            Self::RainbowFish => ("Rainbow_Fish", 38, 80.0),
            Self::Lobster => ("Lobster", 40, 90.0),
            Self::Catfish => ("Catfish", 46, 33.0),
            Self::Bass => ("Bass", 46, 100.0),
            Self::LeapingTrout => ("Leaping_Trout", 48, 50.0),
            Self::Swordfish => ("Swordfish", 50, 100.0),
            Self::LeapingSalmon => ("Leaping_Salmon", 58, 70.0),
            Self::Monkfish => ("Monkfish", 62, 120.0),
            Self::Karambwan => ("Karambwan", 65, 50.0),
            Self::LeapingSturgeon => ("Leaping_Sturgeon", 70, 80.0),
            Self::Shark => ("Shark", 76, 110.0),
            Self::SeaTurtle => ("Sea_Turtle", 79, 38.0),
            Self::InfernalEel => ("Infernal_Eel", 80, 95.0),
            Self::MantaRay => ("Manta_Ray", 81, 46.0),
            Self::Minnow => ("Minnow", 82, 26.5),
            Self::Anglerfish => ("Anglerfish", 82, 120.0),
            Self::DarkCrab => ("Dark_Crab", 85, 130.0),
            Self::SacredEel => ("Sacred_Eel", 87, 105.0),
        };

        Details::Fishing(FishingDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: vec![Multipliers::Fishing(FishingMultipliers::AnglersOutfit)],
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
pub struct FishingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for FishingDetails {
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
                Multipliers::Fishing(x) => x,
                _ => return,
            };
            let d = a.details();
            vec.push(s.p(format!(
                "{} {}",
                s.c1(format!("{}:", d.name.as_str()).as_str()),
                s.c2(common::commas_from_string(
                    format!("{}", (xp_difference / (self.xp * d.value)).ceil()).as_str(),
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
pub enum FishingMultipliers {
    AnglersOutfit,
}

impl FishingMultipliers {
    pub fn details(&self) -> FishingMultiplierDetails {
        let details = match self {
            Self::AnglersOutfit => ("Outfit", 1.025),
        };

        FishingMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct FishingMultiplierDetails {
    pub name: String,
    pub value: f64,
}
