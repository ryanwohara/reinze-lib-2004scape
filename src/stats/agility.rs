use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Agility {
    GnomeStrongholdCourse,
    DraynorVillageRooftop,
    ShayzienBasicCourse,
    LeapingTrout,
    AlKharidRooftop,
    LeapingSalmon,
    VarrockRooftop,
    PenguinAgilityCourse,
    BarbarianOutpost,
    CanifisRooftop,
    LeapingSturgeon,
    ShayzienAdvancedCourse,
    ApeAtollCourse,
    ColossalWyrmBasicCourse,
    FaladorRooftop,
    WildernessAgilityCourseTicket,
    WildernessAgilityCourse,
    HallowedSepulchreFloor1,
    SeersVillageRooftop,
    WerewolfAgilityCourse,
    ColossalWyrmAdvancedCourse,
    HallowedSepulchreFloor2,
    PollnivneachRooftop,
    DorgeshKaanAgilityCourse,
    HallowedSepulchreFloor3,
    PrifddinasAgilityCourse,
    RellekkaRooftop,
    HallowedSepulchreFloor4,
    ArdougneRooftop,
    HallowedSepulchreFloor5,
}

impl Skill for Agility {
    fn defaults() -> Vec<Details> {
        vec![
            Agility::WildernessAgilityCourse,
            Agility::WildernessAgilityCourseTicket,
            Agility::SeersVillageRooftop,
            Agility::PollnivneachRooftop,
            Agility::RellekkaRooftop,
            Agility::ArdougneRooftop,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
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
            Self::DraynorVillageRooftop,
            Self::ShayzienBasicCourse,
            Self::LeapingTrout,
            Self::AlKharidRooftop,
            Self::LeapingSalmon,
            Self::VarrockRooftop,
            Self::PenguinAgilityCourse,
            Self::BarbarianOutpost,
            Self::CanifisRooftop,
            Self::LeapingSturgeon,
            Self::ShayzienAdvancedCourse,
            Self::ApeAtollCourse,
            Self::ColossalWyrmBasicCourse,
            Self::FaladorRooftop,
            Self::WildernessAgilityCourseTicket,
            Self::WildernessAgilityCourse,
            Self::HallowedSepulchreFloor1,
            Self::SeersVillageRooftop,
            Self::WerewolfAgilityCourse,
            Self::ColossalWyrmAdvancedCourse,
            Self::HallowedSepulchreFloor2,
            Self::PollnivneachRooftop,
            Self::DorgeshKaanAgilityCourse,
            Self::HallowedSepulchreFloor3,
            Self::PrifddinasAgilityCourse,
            Self::RellekkaRooftop,
            Self::HallowedSepulchreFloor4,
            Self::ArdougneRooftop,
            Self::HallowedSepulchreFloor5,
        ]
    }

    fn details(&self) -> Details {
        let details = match self {
            Agility::GnomeStrongholdCourse => ("Gnome Stronghold", 1, 110.5, vec![]),
            Agility::DraynorVillageRooftop => ("Draynor Village Rooftop", 1, 120.0, vec![]),
            Agility::ShayzienBasicCourse => ("Shayzien Basic", 1, 153.5, vec![]),
            Agility::LeapingTrout => ("Leaping trout", 15, 5.0, vec![]),
            Agility::AlKharidRooftop => ("Al Kharid Rooftop", 20, 216.0, vec![]),
            Agility::LeapingSalmon => ("Leaping salmon", 30, 6.0, vec![]),
            Agility::VarrockRooftop => ("Varrock Rooftop", 30, 269.7, vec![]),
            Agility::PenguinAgilityCourse => ("Penguin", 30, 540.0, vec![]),
            Agility::BarbarianOutpost => ("Barbarian Outpost", 35, 152.5, vec![]),
            Agility::CanifisRooftop => ("Canifis Rooftop", 40, 240.0, vec![]),
            Agility::LeapingSturgeon => ("Leaping sturgeon", 45, 7.0, vec![]),
            Agility::ShayzienAdvancedCourse => ("Shayzien Advanced", 45, 508.0, vec![]),
            Agility::ApeAtollCourse => ("Ape Atoll", 48, 580.0, vec![]),
            Agility::ColossalWyrmBasicCourse => ("Colossal Wyrm Basic", 50, 504.1, vec![]),
            Agility::FaladorRooftop => ("Falador Rooftop", 50, 586.0, vec![]),
            Agility::WildernessAgilityCourse => ("Wilderness", 52, 571.4, vec![]),
            Agility::WildernessAgilityCourseTicket => (
                "Wilderness Agility Ticket",
                52,
                200.0,
                vec![
                    AgilityMultipliers::WildernessAgilityTicket11To50,
                    AgilityMultipliers::WildernessAgilityTicket51To100,
                    AgilityMultipliers::WildernessAgilityTicket101Plus,
                ],
            ),
            Agility::HallowedSepulchreFloor1 => ("Hallowed Sepulchre Fl 1", 52, 575.0, vec![]),
            Agility::SeersVillageRooftop => ("Seers' Village Rooftop", 60, 570.0, vec![]),
            Agility::WerewolfAgilityCourse => ("Werewolf", 60, 730.0, vec![]),
            Agility::ColossalWyrmAdvancedCourse => ("Colossal Wyrm Advanced", 62, 749.6, vec![]),
            Agility::HallowedSepulchreFloor2 => ("Hallowed Sepulchre Fl 2", 62, 925.0, vec![]),
            Agility::PollnivneachRooftop => (
                "Pollnivneach Rooftop",
                70,
                890.0,
                vec![AgilityMultipliers::DesertHardDiary],
            ),
            Agility::DorgeshKaanAgilityCourse => ("Dorgesh-Kaan", 70, 2750.0, vec![]),
            Agility::HallowedSepulchreFloor3 => ("Hallowed Sepulchre Fl 3", 72, 1600.0, vec![]),
            Agility::PrifddinasAgilityCourse => ("Prifddinas", 75, 1337.0, vec![]),
            Agility::RellekkaRooftop => (
                "Rellekka Rooftop",
                80,
                780.0,
                vec![AgilityMultipliers::FremennikHardDiary],
            ),
            Agility::HallowedSepulchreFloor4 => ("Hallowed Sepulchre Fl 4", 82, 2875.0, vec![]),
            Agility::ArdougneRooftop => ("Ardougne Rooftop", 90, 889.0, vec![]),
            Agility::HallowedSepulchreFloor5 => ("Hallowed Sepulchre Fl 5", 92, 5725.0, vec![]),
        };

        Details::Agility(AgilityDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: details
                .3
                .iter()
                .map(|x| Multipliers::Agility(x.clone()))
                .collect(),
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
        let mut vec = vec![format!(
            "{}: {}",
            s.c1(self.name.as_str()),
            s.c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp).ceil()).as_str(),
                "d"
            )
            .as_str())
        )];

        self.multipliers.iter().for_each(|x| {
            let a = match x {
                Multipliers::Agility(x) => x,
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
pub enum AgilityMultipliers {
    WildernessAgilityTicket11To50,
    WildernessAgilityTicket51To100,
    WildernessAgilityTicket101Plus,
    DesertHardDiary,
    FremennikHardDiary,
}

impl AgilityMultipliers {
    pub fn details(&self) -> AgilityMultiplierDetails {
        let details = match self {
            Self::WildernessAgilityTicket11To50 => ("11-50 Tix", 1.05),
            Self::WildernessAgilityTicket51To100 => ("51-100 Tix", 1.1),
            Self::WildernessAgilityTicket101Plus => ("101+ Tix", 1.15),
            Self::DesertHardDiary => ("Desert Hard Diary", 1016.0 / 890.0),
            Self::FremennikHardDiary => ("Fremmy Hard Diary", 920.0 / 780.0),
        };

        AgilityMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct AgilityMultiplierDetails {
    pub name: String,
    pub value: f64,
}
