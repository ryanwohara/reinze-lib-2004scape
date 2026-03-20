use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Firemaking {
    SacredOil3,
    SacredOil2,
    SacredOil4,
    AcheyTreeLogs,
    Logs,
    PyreLogs,
    OakLogs,
    OakPyreLogs,
    WillowLogs,
    WillowPyreLogs,
    TeakLogs,
    TeakPyreLogs,
    ArcticPineLogs,
    MapleLogs,
    ArcticPyreLogs,
    MahoganyLogs,
    MaplePyreLogs,
    MahoganyPyreLogs,
    YewLogs,
    BlisterwoodLogs,
    YewPyreLogs,
    MagicLogs,
    MagicPyreLogs,
    RedwoodLogs,
    RedwoodPyreLogs,
}

impl Detail for Firemaking {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Firemaking(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Firemaking(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Firemaking(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Firemaking {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::SacredOil3,
            Self::SacredOil2,
            Self::SacredOil4,
            Self::AcheyTreeLogs,
            Self::Logs,
            Self::PyreLogs,
            Self::OakLogs,
            Self::OakPyreLogs,
            Self::WillowLogs,
            Self::WillowPyreLogs,
            Self::TeakLogs,
            Self::TeakPyreLogs,
            Self::ArcticPineLogs,
            Self::MapleLogs,
            Self::ArcticPyreLogs,
            Self::MahoganyLogs,
            Self::MaplePyreLogs,
            Self::MahoganyPyreLogs,
            Self::YewLogs,
            Self::BlisterwoodLogs,
            Self::YewPyreLogs,
            Self::MagicLogs,
            Self::MagicPyreLogs,
            Self::RedwoodLogs,
            Self::RedwoodPyreLogs,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::WillowLogs,
            Self::MapleLogs,
            Self::YewLogs,
            Self::MagicLogs,
            Self::RedwoodLogs,
            Self::RedwoodPyreLogs,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::SacredOil3 => ("Sacred Oil 3", 1, 15.0, FiremakingMethod::SacredOil),
            Self::SacredOil2 => ("Sacred Oil 2", 1, 10.0, FiremakingMethod::SacredOil),
            Self::SacredOil4 => ("Sacred Oil 4", 1, 20.0, FiremakingMethod::SacredOil),
            Self::AcheyTreeLogs => ("Achey Tree Logs", 1, 40.0, FiremakingMethod::NormalLogs),
            Self::Logs => ("Logs", 1, 40.0, FiremakingMethod::NormalLogs),
            Self::PyreLogs => ("Pyre Logs", 5, 50.0, FiremakingMethod::PyreLogs),
            Self::OakLogs => ("Oak Logs", 15, 60.0, FiremakingMethod::NormalLogs),
            Self::OakPyreLogs => ("Oak Pyre Logs", 20, 70.0, FiremakingMethod::PyreLogs),
            Self::WillowLogs => ("Willow Logs", 30, 90.0, FiremakingMethod::NormalLogs),
            Self::WillowPyreLogs => ("Willow Pyre Logs", 35, 100.0, FiremakingMethod::PyreLogs),
            Self::TeakLogs => ("Teak Logs", 35, 105.0, FiremakingMethod::NormalLogs),
            Self::TeakPyreLogs => ("Teak Pyre Logs", 40, 120.0, FiremakingMethod::PyreLogs),
            Self::ArcticPineLogs => ("Arctic Pine Logs", 42, 125.0, FiremakingMethod::NormalLogs),
            Self::MapleLogs => ("Maple Logs", 45, 135.0, FiremakingMethod::NormalLogs),
            Self::ArcticPyreLogs => ("Arctic Pyre Logs", 47, 158.0, FiremakingMethod::PyreLogs),
            Self::MahoganyLogs => ("Mahogany Logs", 50, 157.5, FiremakingMethod::NormalLogs),
            Self::MaplePyreLogs => ("Maple Pyre Logs", 50, 175.0, FiremakingMethod::PyreLogs),
            Self::MahoganyPyreLogs => ("Mahogany Pyre Logs", 55, 210.0, FiremakingMethod::PyreLogs),
            Self::YewLogs => ("Yew Logs", 60, 202.5, FiremakingMethod::NormalLogs),
            Self::BlisterwoodLogs => ("Blisterwood Logs", 62, 96.0, FiremakingMethod::NormalLogs),
            Self::YewPyreLogs => ("Yew Pyre Logs", 65, 255.0, FiremakingMethod::PyreLogs),
            Self::MagicLogs => ("Magic Logs", 75, 303.8, FiremakingMethod::NormalLogs),
            Self::MagicPyreLogs => ("Magic Pyre Logs", 80, 404.5, FiremakingMethod::PyreLogs),
            Self::RedwoodLogs => ("Redwood Logs", 90, 350.0, FiremakingMethod::NormalLogs),
            Self::RedwoodPyreLogs => ("Redwood Pyre Logs", 95, 500.0, FiremakingMethod::PyreLogs),
        };

        Details::Firemaking(FiremakingDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: match details.3 {
                FiremakingMethod::NormalLogs => vec![
                    Multipliers::Firemaking(FiremakingMultipliers::PyromancerOutfit),
                    Multipliers::Firemaking(FiremakingMultipliers::ForestersCampfire),
                    Multipliers::Firemaking(FiremakingMultipliers::BothOutfitAndCampfire),
                ],
                FiremakingMethod::PyreLogs => vec![Multipliers::Firemaking(
                    FiremakingMultipliers::MorytaniaElite,
                )],
                FiremakingMethod::SacredOil => vec![],
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

pub enum FiremakingMethod {
    NormalLogs,
    PyreLogs,
    SacredOil,
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct FiremakingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for FiremakingDetails {
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
                Multipliers::Firemaking(x) => x,
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
pub enum FiremakingMultipliers {
    BothOutfitAndCampfire,
    PyromancerOutfit,
    ForestersCampfire,
    MorytaniaElite,
}

impl FiremakingMultipliers {
    pub fn details(&self) -> FiremakingMultiplierDetails {
        let details = match self {
            Self::BothOutfitAndCampfire => ("Both", 1.025 * 0.33333333333),
            Self::PyromancerOutfit => ("Outfit", 1.025),
            Self::ForestersCampfire => ("Campfire", 0.33333333333),
            Self::MorytaniaElite => ("Morytania Elite", 1.5),
        };

        FiremakingMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct FiremakingMultiplierDetails {
    pub name: String,
    pub value: f64,
}
