use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Cooking {
    CookedMeat,
    Shrimp,
    Bread,
    Sardine,
    Herring,
    Mackerel,
    RedberryPie,
    Anchovy,
    Trout,
    Cod,
    Pike,
    MeatPie,
    Salmon,
    Stew,
    ChompyBird,
    Tuna,
    ApplePie,
    Wine,
    Pizza,
    Lobster,
    Cake,
    Bass,
    Swordfish,
    MeatPizza,
    ChocolateCake,
    OomlieWrap,
    LavaEel,
    AnchovyPizza,
    UgthankiKebab,
    Curry,
    PineapplePizza,
    Shark,
    SeaTurtle,
    MantaRay,
}

impl Skill for Cooking {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::CookedMeat,
            Self::Shrimp,
            Self::Bread,
            Self::Sardine,
            Self::Herring,
            Self::Mackerel,
            Self::RedberryPie,
            Self::Anchovy,
            Self::Trout,
            Self::Cod,
            Self::Pike,
            Self::MeatPie,
            Self::Salmon,
            Self::Stew,
            Self::ChompyBird,
            Self::Tuna,
            Self::ApplePie,
            Self::Wine,
            Self::Pizza,
            Self::Lobster,
            Self::Cake,
            Self::Bass,
            Self::Swordfish,
            Self::MeatPizza,
            Self::ChocolateCake,
            Self::OomlieWrap,
            Self::LavaEel,
            Self::AnchovyPizza,
            Self::UgthankiKebab,
            Self::Curry,
            Self::PineapplePizza,
            Self::Shark,
            Self::SeaTurtle,
            Self::MantaRay,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![Self::Lobster, Self::Swordfish, Self::Shark, Self::MantaRay]
            .iter()
            .map(|x| x.details())
            .collect()
    }

    fn details(&self) -> Details {
        //                    (name, level, heals, bites, xp)
        let details = match self {
            Self::CookedMeat => ("Cooked Meat", 1, 3, 1, 30.0),
            Self::Shrimp => ("Shrimp", 1, 3, 1, 30.0),
            Self::Bread => ("Bread", 1, 4, 1, 40.0),
            Self::Sardine => ("Sardine", 1, 4, 1, 40.0),
            Self::Herring => ("Herring", 5, 5, 1, 50.0),
            Self::Mackerel => ("Mackerel", 10, 6, 1, 60.0),
            Self::RedberryPie => ("Redberry Pie", 10, 6, 2, 60.0),
            Self::Anchovy => ("Anchovy", 15, 3, 1, 30.0),
            Self::Trout => ("Trout", 15, 7, 1, 70.0),
            Self::Cod => ("Cod", 18, 7, 1, 70.0),
            Self::Pike => ("Pike", 20, 8, 1, 80.0),
            Self::MeatPie => ("Meat Pie", 20, 8, 2, 80.0),
            Self::Salmon => ("Salmon", 25, 9, 1, 90.0),
            Self::Stew => ("Stew", 25, 9, 1, 90.0),
            Self::ChompyBird => ("Chompy Bird", 30, 10, 1, 100.0),
            Self::Tuna => ("Tuna", 30, 10, 1, 100.0),
            Self::ApplePie => ("Apple Pie", 30, 10, 2, 100.0),
            Self::Wine => ("Wine", 35, 15, 1, 110.0),
            Self::Pizza => ("Pizza", 35, 10, 1, 110.0),
            Self::Lobster => ("Lobster", 40, 12, 1, 120.0),
            Self::Cake => ("Cake", 40, 12, 3, 120.0),
            Self::Bass => ("Bass", 43, 13, 1, 130.0),
            Self::Swordfish => ("Swordfish", 45, 14, 1, 140.0),
            Self::MeatPizza => ("Meat Pizza", 45, 14, 2, 140.0),
            Self::ChocolateCake => ("Chocolate Cake", 50, 15, 3, 120.0),
            Self::OomlieWrap => ("Oomlie Wrap", 50, 14, 1, 40.0),
            Self::LavaEel => ("Lava Eel", 53, 14, 1, 140.0),
            Self::AnchovyPizza => ("Anchovy Pizza", 55, 16, 2, 140.0),
            Self::UgthankiKebab => ("Ugthanki Kebab", 53, 19, 1, 120.0),
            Self::Curry => ("Curry", 60, 19, 1, 125.0),
            Self::PineapplePizza => ("Pineapple Pizza", 65, 20, 2, 110.0),
            Self::Shark => ("Shark", 80, 20, 1, 210.0),
            Self::SeaTurtle => ("Sea Turtle", 82, 20, 1, 0.0),
            Self::MantaRay => ("Manta Ray", 91, 22, 1, 0.0),
        };

        Details::Cooking(CookingDetails {
            name: details.0.to_owned(),
            level: details.1,
            heals: details.2,
            bites: details.3,
            xp: details.4,
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

impl Detail for Cooking {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Cooking(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Cooking(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Cooking(obj) = self.details() {
            return obj.xp;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct CookingDetails {
    pub name: String,
    pub level: u32,
    pub heals: u32,
    pub bites: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for CookingDetails {
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
