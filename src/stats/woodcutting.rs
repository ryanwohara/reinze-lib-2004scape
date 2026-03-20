use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Woodcutting {
    AcheyTreeLogs,
    Logs,
    OakLogs,
    WillowLogs,
    TeakLogs,
    JatobaLogs,
    JuniperLogs,
    Bark,
    MapleLogs,
    MahoganyLogs,
    ArcticPineLogs,
    CamphorLogs,
    YewLogs,
    BlisterwoodLogs,
    Sulliusceps,
    MagicLogs,
    IronwoodLogs,
    RedwoodLogs,
    RosewoodLogs,
}

impl Skill for Woodcutting {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::AcheyTreeLogs,
            Self::Logs,
            Self::OakLogs,
            Self::WillowLogs,
            Self::TeakLogs,
            Self::JatobaLogs,
            Self::JuniperLogs,
            Self::Bark,
            Self::MapleLogs,
            Self::MahoganyLogs,
            Self::ArcticPineLogs,
            Self::CamphorLogs,
            Self::YewLogs,
            Self::BlisterwoodLogs,
            Self::Sulliusceps,
            Self::MagicLogs,
            Self::IronwoodLogs,
            Self::RedwoodLogs,
            Self::RosewoodLogs,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::WillowLogs,
            Self::MapleLogs,
            Self::YewLogs,
            Self::MagicLogs,
            Self::IronwoodLogs,
            Self::RedwoodLogs,
            Self::RosewoodLogs,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::AcheyTreeLogs => ("Achey Tree Logs", 1, 25.0),
            Self::Logs => ("Logs", 1, 25.0),
            Self::OakLogs => ("Oak Logs", 15, 37.5),
            Self::WillowLogs => ("Willow Logs", 30, 67.5),
            Self::TeakLogs => ("Teak Logs", 35, 85.0),
            Self::JatobaLogs => ("Jatoba Logs", 40, 92.0),
            Self::JuniperLogs => ("Juniper Logs", 42, 35.0),
            Self::Bark => ("Bark", 45, 82.5),
            Self::MapleLogs => ("Maple Logs", 45, 100.0),
            Self::MahoganyLogs => ("Mahogany Logs", 50, 125.0),
            Self::ArcticPineLogs => ("Arctic Pine Logs", 54, 40.0),
            Self::CamphorLogs => ("Camphor Logs", 66, 143.0),
            Self::YewLogs => ("Yew Logs", 60, 175.0),
            Self::BlisterwoodLogs => ("Blisterwood Logs", 62, 76.0),
            Self::Sulliusceps => ("Sulliusceps", 65, 127.0),
            Self::MagicLogs => ("Magic Logs", 75, 250.0),
            Self::IronwoodLogs => ("Ironwood Logs", 80, 175.0),
            Self::RedwoodLogs => ("Redwood Logs", 90, 380.0),
            Self::RosewoodLogs => ("Rosewood Logs", 92, 212.5),
        };

        Details::Woodcutting(WoodcuttingDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: vec![
                Multipliers::Woodcutting(WoodcuttingMultipliers::LumberjackOutfit),
                Multipliers::Woodcutting(WoodcuttingMultipliers::FellingAxeAndRations),
                Multipliers::Woodcutting(WoodcuttingMultipliers::Both),
            ],
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

impl Detail for Woodcutting {
    fn multipliers(&self) -> Vec<Multipliers> {
        if let Details::Woodcutting(obj) = self.details() {
            return obj.multipliers;
        }

        vec![]
    }

    fn name(&self) -> String {
        if let Details::Woodcutting(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Woodcutting(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Woodcutting(obj) = self.details() {
            return obj.xp;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct WoodcuttingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for WoodcuttingDetails {
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
                Multipliers::Woodcutting(x) => x,
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
pub enum WoodcuttingMultipliers {
    LumberjackOutfit,
    FellingAxeAndRations,
    Both,
}

impl WoodcuttingMultipliers {
    pub fn details(&self) -> WoodcuttingMultiplierDetails {
        let details = match self {
            Self::LumberjackOutfit => ("Outfit", 1.025),
            Self::FellingAxeAndRations => ("FA&Rations", 1.1),
            Self::Both => ("Both", 1.025 * 1.1),
        };

        WoodcuttingMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct WoodcuttingMultiplierDetails {
    pub name: String,
    pub value: f64,
}
