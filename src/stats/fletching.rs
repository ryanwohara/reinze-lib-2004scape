use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Fletching {
    ArrowShaft,
    HeadlessArrow,
    BronzeArrow,
    BronzeJavelin,
    OgreArrow,
    Shortbow,
    ShortbowU,
    BronzeDart,
    Longbow,
    LongbowU,
    IronArrow,
    IronJavelin,
    OakShortbow,
    OakShortbowU,
    IronDart,
    SteelArrow,
    SteelJavelin,
    WillowShortbow,
    WillowShortbowU,
    SteelDart,
    WillowLongbow,
    WillowLongbowU,
    MithrilArrow,
    MithrilJavelin,
    MapleShortbow,
    MapleShortbowU,
    MithrilDart,
    MapleLongbow,
    MapleLongbowU,
    AdamantArrow,
    AdamantJavelin,
    YewShortbow,
    YewShortbowU,
    AdamantDart,
    YewLongbow,
    YewLongbowU,
    RuneArrow,
    RuneJavelin,
    MagicShortbow,
    MagicShortbowU,
    RuneDart,
    MagicLongbow,
    MagicLongbowU,
}

impl Detail for Fletching {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Fletching(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Fletching(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Fletching(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Fletching {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::ArrowShaft,
            Self::HeadlessArrow,
            Self::BronzeArrow,
            Self::BronzeJavelin,
            Self::OgreArrow,
            Self::Shortbow,
            Self::ShortbowU,
            Self::BronzeDart,
            Self::Longbow,
            Self::LongbowU,
            Self::IronArrow,
            Self::IronJavelin,
            Self::OakShortbow,
            Self::OakShortbowU,
            Self::IronDart,
            Self::SteelArrow,
            Self::SteelJavelin,
            Self::WillowShortbow,
            Self::WillowShortbowU,
            Self::SteelDart,
            Self::WillowLongbow,
            Self::WillowLongbowU,
            Self::MithrilArrow,
            Self::MithrilJavelin,
            Self::MapleShortbow,
            Self::MapleShortbowU,
            Self::MithrilDart,
            Self::MapleLongbow,
            Self::MapleLongbowU,
            Self::AdamantArrow,
            Self::AdamantJavelin,
            Self::YewShortbow,
            Self::YewShortbowU,
            Self::AdamantDart,
            Self::YewLongbow,
            Self::YewLongbowU,
            Self::RuneArrow,
            Self::RuneJavelin,
            Self::MagicShortbow,
            Self::MagicShortbowU,
            Self::RuneDart,
            Self::MagicLongbow,
            Self::MagicLongbowU,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::MapleShortbowU,
            Self::MapleLongbowU,
            Self::YewShortbowU,
            Self::YewLongbowU,
            Self::MagicShortbowU,
            Self::MagicLongbowU,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::ArrowShaft => ("Arrow_Shaft", 1, 0.33),
            Self::HeadlessArrow => ("Headless_Arrow", 1, 1.0),
            Self::BronzeArrow => ("Bronze_Arrow", 1, 1.3),
            Self::BronzeJavelin => ("Bronze_Javelin", 3, 1.0),
            Self::OgreArrow => ("Ogre_Arrow", 5, 1.0),
            Self::Shortbow => ("Shortbow", 5, 5.0),
            Self::ShortbowU => ("Shortbow_U", 5, 5.0),
            Self::BronzeDart => ("Bronze_Dart", 10, 1.8),
            Self::Longbow => ("Longbow", 10, 10.0),
            Self::LongbowU => ("Longbow_U", 10, 10.0),
            Self::IronArrow => ("Iron_Arrow", 15, 2.5),
            Self::IronJavelin => ("Iron_Javelin", 17, 2.0),
            Self::OakShortbow => ("Oak_Shortbow", 20, 16.5),
            Self::OakShortbowU => ("Oak_Shortbow_U", 20, 16.5),
            Self::IronDart => ("Iron_Dart", 22, 3.8),
            Self::SteelArrow => ("Steel_Arrow", 30, 5.0),
            Self::SteelJavelin => ("Steel_Javelin", 32, 5.0),
            Self::WillowShortbow => ("Willow_Shortbow", 35, 33.3),
            Self::WillowShortbowU => ("Willow_Shortbow_U", 35, 33.3),
            Self::SteelDart => ("Steel_Dart", 37, 7.5),
            Self::WillowLongbow => ("Willow_Longbow", 40, 41.5),
            Self::WillowLongbowU => ("Willow_Longbow_U", 40, 41.5),
            Self::MithrilArrow => ("Mithril_Arrow", 45, 7.5),
            Self::MithrilJavelin => ("Mithril_Javelin", 47, 8.0),
            Self::MapleShortbow => ("Maple_Shortbow", 50, 50.0),
            Self::MapleShortbowU => ("Maple_Shortbow_U", 50, 50.0),
            Self::MithrilDart => ("Mithril_Dart", 52, 11.2),
            Self::MapleLongbow => ("Maple_Longbow", 55, 58.2),
            Self::MapleLongbowU => ("Maple_Longbow_U", 55, 58.3),
            Self::AdamantArrow => ("Adamant_Arrow", 60, 10.0),
            Self::AdamantJavelin => ("Adamant_Javelin", 62, 10.0),
            Self::YewShortbow => ("Yew_Shortbow", 65, 67.5),
            Self::YewShortbowU => ("Yew_Shortbow_U", 65, 67.5),
            Self::AdamantDart => ("Adamant_Dart", 67, 15.0),
            Self::YewLongbow => ("Yew_Longbow", 70, 75.0),
            Self::YewLongbowU => ("Yew_Longbow_U", 70, 75.0),
            Self::RuneArrow => ("Rune_Arrow", 75, 12.5),
            Self::RuneJavelin => ("Rune_Javelin", 77, 12.4),
            Self::MagicShortbow => ("Magic_Shortbow", 80, 83.3),
            Self::MagicShortbowU => ("Magic_Shortbow_U", 80, 83.3),
            Self::RuneDart => ("Rune_Dart", 81, 18.8),
            Self::MagicLongbow => ("Magic_Longbow", 85, 91.5),
            Self::MagicLongbowU => ("Magic_Longbow_U", 85, 91.5),
        };

        Details::Fletching(FletchingDetails {
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

#[derive(Clone, PartialOrd, PartialEq)]
pub struct FletchingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
}

impl IntoString for FletchingDetails {
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
