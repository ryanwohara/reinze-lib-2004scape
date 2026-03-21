use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Mining {
    Clay,
    Copper,
    Tin,
    Blurite,
    Iron,
    Silver,
    Coal,
    Gold,
    Mithril,
    Adamantite,
    Runite,
}

impl Skill for Mining {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Clay,
            Self::Copper,
            Self::Tin,
            Self::Blurite,
            Self::Iron,
            Self::Silver,
            Self::Coal,
            Self::Gold,
            Self::Mithril,
            Self::Adamantite,
            Self::Runite,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::Iron,
            Self::Coal,
            Self::Gold,
            Self::Mithril,
            Self::Adamantite,
            Self::Runite,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::Clay => ("Clay", 1, 5.0),
            Self::Copper => ("Copper", 1, 17.5),
            Self::Tin => ("Tin", 1, 17.5),
            Self::Blurite => ("Blurite", 10, 17.5),
            Self::Iron => ("Iron", 15, 35.0),
            Self::Silver => ("Silver", 20, 40.0),
            Self::Coal => ("Coal", 30, 50.0),
            Self::Gold => ("Gold", 40, 65.0),
            Self::Mithril => ("Mithril", 55, 80.0),
            Self::Adamantite => ("Adamantite", 70, 95.0),
            Self::Runite => ("Runite", 85, 120.0),
        };

        Details::Mining(MiningDetails {
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

impl Detail for Mining {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Mining(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Mining(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Mining(obj) = self.details() {
            return obj.xp;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct MiningDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
}

impl IntoString for MiningDetails {
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
