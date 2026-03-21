use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Prayer {
    Bones,
    WolfBones,
    BurntBones,
    BatBones,
    BigBones,
    BabydragonBones,
    DragonBones,
}

impl Detail for Prayer {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Prayer(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Prayer(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Prayer(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Prayer {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Bones,
            Self::WolfBones,
            Self::BurntBones,
            Self::BatBones,
            Self::BigBones,
            Self::BabydragonBones,
            Self::DragonBones,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![Self::BigBones, Self::BabydragonBones, Self::DragonBones]
            .iter()
            .map(|x| x.details())
            .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::Bones => ("Bones", 1, 4.5),
            Self::WolfBones => ("Wolf Bones", 1, 4.5),
            Self::BurntBones => ("Burnt Bones", 1, 4.5),
            Self::BatBones => ("Bat Bones", 1, 5.25),
            Self::BigBones => ("Big Bones", 1, 15.0),
            Self::BabydragonBones => ("Babydragon Bones", 1, 30.0),
            Self::DragonBones => ("Dragon Bones", 1, 72.0),
        };

        Details::Prayer(PrayerDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: vec![Multipliers::Prayer(PrayerMultipliers::Ectofuntus)],
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
pub struct PrayerDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for PrayerDetails {
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
                Multipliers::Prayer(x) => x,
                _ => return,
            };
            let d = a.details();
            vec.push(
                s.p(format!(
                    "{} {}",
                    s.c1(format!("{}:", d.name.as_str()).as_str()),
                    s.c2(common::commas_from_string(
                        format!("{}", (xp_difference / (self.xp as f64 * d.value)).ceil()).as_str(),
                        "d"
                    )
                    .as_str())
                )
                .as_str()),
            );
        });

        vec.join(" ")
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PrayerMultipliers {
    Ectofuntus,
}

impl PrayerMultipliers {
    pub fn details(&self) -> PrayerMultiplierDetails {
        let details = match self {
            Self::Ectofuntus => ("Ectofuntus", 4.0),
        };

        PrayerMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct PrayerMultiplierDetails {
    pub name: String,
    pub value: f64,
}
