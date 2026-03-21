use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Fishing {
    Shrimps,
    Karambwanji,
    Sardine,
    Herring,
    Anchovies,
    Mackerel,
    Trout,
    Cod,
    Pike,
    SlimyEel,
    Salmon,
    Tuna,
    Lobster,
    Bass,
    Swordfish,
    Karambwan,
    Shark,
    SeaTurtle,
    MantaRay,
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
            Self::Herring,
            Self::Anchovies,
            Self::Mackerel,
            Self::Trout,
            Self::Cod,
            Self::Pike,
            Self::SlimyEel,
            Self::Salmon,
            Self::Tuna,
            Self::Lobster,
            Self::Bass,
            Self::Swordfish,
            Self::Karambwan,
            Self::Shark,
            Self::SeaTurtle,
            Self::MantaRay,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::Trout,
            Self::Salmon,
            Self::Tuna,
            Self::Lobster,
            Self::Swordfish,
            Self::Shark,
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
            Self::Herring => ("Herring", 10, 30.0),
            Self::Anchovies => ("Anchovies", 15, 40.0),
            Self::Mackerel => ("Mackerel", 16, 20.0),
            Self::Trout => ("Trout", 20, 50.0),
            Self::Cod => ("Cod", 23, 45.0),
            Self::Pike => ("Pike", 25, 60.0),
            Self::SlimyEel => ("Slimy_Eel", 28, 80.0),
            Self::Salmon => ("Salmon", 30, 70.0),
            Self::Tuna => ("Tuna", 35, 80.0),
            Self::Lobster => ("Lobster", 40, 90.0),
            Self::Bass => ("Bass", 46, 100.0),
            Self::Swordfish => ("Swordfish", 50, 100.0),
            Self::Karambwan => ("Karambwan", 65, 50.0),
            Self::Shark => ("Shark", 76, 110.0),
            Self::SeaTurtle => ("Sea_Turtle", 79, 38.0),
            Self::MantaRay => ("Manta_Ray", 81, 46.0),
        };

        Details::Fishing(FishingDetails {
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

#[derive(Clone, PartialOrd, PartialEq)]
pub struct FishingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for FishingDetails {
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
