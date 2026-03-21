use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Agility {
    GnomeStrongholdCourse,
    BarbarianOutpost,
    Wilderness,
    // Brimhaven Agility Arena Tickets
    OneTicket,
    TenTickets,
    TwentyFiveTickets,
    OneHundredTickets,
    OneThousandTickets,
}

impl Skill for Agility {
    fn defaults() -> Vec<Details> {
        Self::all().iter().map(|x| x.details()).collect()
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
                return if pattern
                    .captures(activity.name().to_lowercase().as_str())
                    .iter()
                    .count()
                    > 0
                    && index < 10
                {
                    index = index.add(1);

                    true
                } else {
                    false
                };
            });
        } else {
            return vec![];
        }

        all
    }

    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::GnomeStrongholdCourse,
            Self::BarbarianOutpost,
            Self::Wilderness,
            Self::OneTicket,
            Self::TenTickets,
            Self::TwentyFiveTickets,
            Self::OneHundredTickets,
            Self::OneThousandTickets,
        ]
    }

    fn details(&self) -> Details {
        let details = match self {
            Agility::GnomeStrongholdCourse => ("Gnome Stronghold", 1, 92.5),
            Agility::BarbarianOutpost => ("Barbarian Outpost", 35, 139.5),
            Agility::Wilderness => ("Wilderness", 52, 682.0),
            // Brimhaven Agility Arena Tickets
            Agility::OneTicket => ("One Ticket", 1, 240.0),
            Agility::TenTickets => ("Ten Tickets", 1, 2480.0),
            Agility::TwentyFiveTickets => ("Twenty Five Tickets", 1, 6500.0),
            Agility::OneHundredTickets => ("One Hundred Tickets", 1, 28000.0),
            Agility::OneThousandTickets => ("One Thousand Tickets", 1, 320000.0),
        };

        Details::Agility(AgilityDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: vec![],
        })
    }
}

impl Detail for Agility {
    fn multipliers(&self) -> Vec<Multipliers> {
        if let Details::Agility(obj) = self.details() {
            return obj.multipliers;
        }

        vec![]
    }

    fn name(&self) -> String {
        if let Details::Agility(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Agility(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Agility(obj) = self.details() {
            return obj.xp;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct AgilityDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for AgilityDetails {
    fn to_string(&self, s: &crate::stats::skill::Source, xp_difference: f64) -> String {
        vec![format!(
            "{}: {}",
            s.c1(self.name.as_str()),
            s.c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp).ceil()).as_str(),
                "d"
            )
            .as_str())
        )]
        .join(" ")
    }
}
