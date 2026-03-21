use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Firemaking {
    Regular,
    Oak,
    Willow,
    Maple,
    Yew,
    Magic,
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
            Self::Regular,
            Self::Oak,
            Self::Willow,
            Self::Maple,
            Self::Yew,
            Self::Magic,
        ]
    }

    fn defaults() -> Vec<Details> {
        Self::all().iter().map(|x| x.details()).collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::Regular => ("Regular", 1, 40.0),
            Self::Oak => ("Oak", 15, 60.0),
            Self::Willow => ("Willow", 30, 90.0),
            Self::Maple => ("Maple", 45, 135.0),
            Self::Yew => ("Yew", 60, 202.5),
            Self::Magic => ("Magic", 75, 303.0),
        };

        Details::Firemaking(FiremakingDetails {
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
pub struct FiremakingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for FiremakingDetails {
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
