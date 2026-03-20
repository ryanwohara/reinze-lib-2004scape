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
    BronzeBolts,
    BronzeCrossbow,
    WoodenStock,
    BronzeCrossbowU,
    BronzeDart,
    Longbow,
    LongbowU,
    OpalBolts,
    IronArrow,
    IronJavelin,
    OakShortbow,
    OakShortbowU,
    IronDart,
    BluriteCrossbow,
    OakStock,
    BluriteCrossbowU,
    OakLongbow,
    OakLongbowU,
    OakShield,
    SteelArrow,
    KebbitBolts,
    SteelJavelin,
    WillowShortbow,
    WillowShortbowU,
    SteelDart,
    IronBolts,
    IronCrossbow,
    WillowStock,
    IronCrossbowU,
    WillowLongbow,
    WillowLongbowU,
    Battlestaff,
    PearlBolts,
    LongKebbitBolts,
    WillowShield,
    SilverBolts,
    MithrilArrow,
    SteelBolts,
    SteelCrossbow,
    TeakStock,
    SteelCrossbowU,
    MithrilJavelin,
    MapleShortbow,
    MapleShortbowU,
    BarbedBolts,
    BroadArrows,
    MithrilDart,
    MithrilBolts,
    MapleStock,
    MithrilCrossbow,
    MithrilCrossbowU,
    BroadBolts,
    MapleLongbow,
    MapleLongbowU,
    SapphireBolts,
    MapleShield,
    EmeraldBolts,
    HuntersSpear,
    AdamantArrow,
    AdamantBolts,
    AdamantCrossbow,
    MahoganyStock,
    AdamantCrossbowU,
    AdamantJavelin,
    RubyBolts,
    DiamondBolts,
    YewShortbow,
    YewShortbowU,
    AdamantDart,
    RuniteBolts,
    RuneCrossbow,
    YewStock,
    RuniteCrossbowU,
    YewLongbow,
    YewLongbowU,
    DragonstoneBolts,
    YewShield,
    OnyxBolts,
    RuneArrow,
    AmethystBroadBolts,
    RuneJavelin,
    DragonCrossbow,
    MagicStock,
    ToxicBlowpipe,
    DragonCrossbowU,
    MagicShortbow,
    MagicShortbowU,
    RuneDart,
    AmethystArrow,
    DragonBolts,
    AmethystJavelin,
    MagicLongbow,
    MagicLongbowU,
    MagicShield,
    DragonArrow,
    AmethystDart,
    DragonJavelin,
    RedwoodShield,
    DragonDart,
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
            Self::BronzeBolts,
            Self::BronzeCrossbow,
            Self::WoodenStock,
            Self::BronzeCrossbowU,
            Self::BronzeDart,
            Self::Longbow,
            Self::LongbowU,
            Self::OpalBolts,
            Self::IronArrow,
            Self::IronJavelin,
            Self::OakShortbow,
            Self::OakShortbowU,
            Self::IronDart,
            Self::BluriteCrossbow,
            Self::OakStock,
            Self::BluriteCrossbowU,
            Self::OakLongbow,
            Self::OakLongbowU,
            Self::OakShield,
            Self::SteelArrow,
            Self::KebbitBolts,
            Self::SteelJavelin,
            Self::WillowShortbow,
            Self::WillowShortbowU,
            Self::SteelDart,
            Self::IronBolts,
            Self::IronCrossbow,
            Self::WillowStock,
            Self::IronCrossbowU,
            Self::WillowLongbow,
            Self::WillowLongbowU,
            Self::Battlestaff,
            Self::PearlBolts,
            Self::LongKebbitBolts,
            Self::WillowShield,
            Self::SilverBolts,
            Self::MithrilArrow,
            Self::SteelBolts,
            Self::SteelCrossbow,
            Self::TeakStock,
            Self::SteelCrossbowU,
            Self::MithrilJavelin,
            Self::MapleShortbow,
            Self::MapleShortbowU,
            Self::BarbedBolts,
            Self::BroadArrows,
            Self::MithrilDart,
            Self::MithrilBolts,
            Self::MapleStock,
            Self::MithrilCrossbow,
            Self::MithrilCrossbowU,
            Self::BroadBolts,
            Self::MapleLongbow,
            Self::MapleLongbowU,
            Self::SapphireBolts,
            Self::MapleShield,
            Self::EmeraldBolts,
            Self::HuntersSpear,
            Self::AdamantArrow,
            Self::AdamantBolts,
            Self::AdamantCrossbow,
            Self::MahoganyStock,
            Self::AdamantCrossbowU,
            Self::AdamantJavelin,
            Self::RubyBolts,
            Self::DiamondBolts,
            Self::YewShortbow,
            Self::YewShortbowU,
            Self::AdamantDart,
            Self::RuniteBolts,
            Self::RuneCrossbow,
            Self::YewStock,
            Self::RuniteCrossbowU,
            Self::YewLongbow,
            Self::YewLongbowU,
            Self::DragonstoneBolts,
            Self::YewShield,
            Self::OnyxBolts,
            Self::RuneArrow,
            Self::AmethystBroadBolts,
            Self::RuneJavelin,
            Self::DragonCrossbow,
            Self::MagicStock,
            Self::ToxicBlowpipe,
            Self::DragonCrossbowU,
            Self::MagicShortbow,
            Self::MagicShortbowU,
            Self::RuneDart,
            Self::AmethystArrow,
            Self::DragonBolts,
            Self::AmethystJavelin,
            Self::MagicLongbow,
            Self::MagicLongbowU,
            Self::MagicShield,
            Self::DragonArrow,
            Self::AmethystDart,
            Self::DragonJavelin,
            Self::RedwoodShield,
            Self::DragonDart,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::MapleShortbowU,
            Self::BroadBolts,
            Self::MapleLongbowU,
            Self::YewShortbowU,
            Self::YewLongbowU,
            Self::MagicShortbowU,
            Self::MagicShortbow,
            Self::MagicLongbowU,
            Self::MagicLongbow,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::ArrowShaft => ("Arrow Shaft", 1, 0.33),
            Self::HeadlessArrow => ("Headless Arrow", 1, 1.0),
            Self::BronzeArrow => ("Bronze Arrow", 1, 1.3),
            Self::BronzeJavelin => ("Bronze Javelin", 3, 1.0),
            Self::OgreArrow => ("Ogre Arrow", 5, 1.0),
            Self::Shortbow => ("Shortbow", 5, 5.0),
            Self::ShortbowU => ("Shortbow U", 5, 5.0),
            Self::BronzeBolts => ("Bronze Bolts", 9, 0.5),
            Self::BronzeCrossbow => ("Bronze Crossbow", 9, 6.0),
            Self::WoodenStock => ("Wooden Stock", 9, 6.0),
            Self::BronzeCrossbowU => ("Bronze CrossbowU", 9, 12.0),
            Self::BronzeDart => ("Bronze Dart", 10, 1.8),
            Self::Longbow => ("Longbow", 10, 10.0),
            Self::LongbowU => ("Longbow U", 10, 10.0),
            Self::OpalBolts => ("Opal Bolts", 11, 1.6),
            Self::IronArrow => ("Iron Arrow", 15, 2.5),
            Self::IronJavelin => ("Iron Javelin", 17, 2.0),
            Self::OakShortbow => ("Oak Shortbow", 20, 16.5),
            Self::OakShortbowU => ("Oak ShortbowU", 20, 16.5),
            Self::IronDart => ("Iron Dart", 22, 3.8),
            Self::BluriteCrossbow => ("Blurite Crossbow", 24, 16.0),
            Self::OakStock => ("Oak Stock", 24, 16.0),
            Self::BluriteCrossbowU => ("Blurite Crossbow U", 24, 32.0),
            Self::OakLongbow => ("Oak Longbow", 25, 25.0),
            Self::OakLongbowU => ("Oak Longbow U", 25, 25.0),
            Self::OakShield => ("Oak Shield", 27, 50.0),
            Self::SteelArrow => ("Steel Arrow", 30, 5.0),
            Self::KebbitBolts => ("Kebbit Bolts", 32, 1.0),
            Self::SteelJavelin => ("Steel Javelin", 32, 5.0),
            Self::WillowShortbow => ("Willow Shortbow", 35, 33.3),
            Self::WillowShortbowU => ("Willow ShortbowU", 35, 33.3),
            Self::SteelDart => ("Steel Dart", 37, 7.5),
            Self::IronBolts => ("Iron Bolts", 39, 1.5),
            Self::IronCrossbow => ("Iron Crossbow", 39, 22.0),
            Self::WillowStock => ("Willow Stock", 39, 22.0),
            Self::IronCrossbowU => ("Iron Crossbow U", 39, 44.0),
            Self::WillowLongbow => ("Willow Longbow", 40, 41.5),
            Self::WillowLongbowU => ("Willow LongbowU", 40, 41.5),
            Self::Battlestaff => ("Battlestaff", 40, 80.0),
            Self::PearlBolts => ("Pearl Bolts", 41, 3.2),
            Self::LongKebbitBolts => ("Long Kebbit Bolts", 42, 1.3),
            Self::WillowShield => ("Willow Shield", 42, 83.0),
            Self::SilverBolts => ("Silver Bolts", 43, 2.5),
            Self::MithrilArrow => ("Mithril Arrow", 45, 7.5),
            Self::SteelBolts => ("Steel Bolts", 46, 3.5),
            Self::SteelCrossbow => ("Steel Crossbow", 46, 27.0),
            Self::TeakStock => ("Teak Stock", 46, 27.0),
            Self::SteelCrossbowU => ("Steel Crossbow U", 46, 54.0),
            Self::MithrilJavelin => ("Mithril Javelin", 47, 8.0),
            Self::MapleShortbow => ("Maple Shortbow", 50, 50.0),
            Self::MapleShortbowU => ("Maple Shortbow U", 50, 50.0),
            Self::BarbedBolts => ("Barbed Bolts", 51, 9.5),
            Self::BroadArrows => ("Broad Arrows", 52, 10.0),
            Self::MithrilDart => ("Mithril Dart", 52, 11.2),
            Self::MithrilBolts => ("Mithril Bolts", 54, 5.0),
            Self::MapleStock => ("Maple Stock", 54, 32.0),
            Self::MithrilCrossbow => ("Mithril Crossbow", 54, 32.0),
            Self::MithrilCrossbowU => ("Mithril Crossbow U", 54, 64.0),
            Self::BroadBolts => ("Broad Bolts", 55, 3.0),
            Self::MapleLongbow => ("Maple Longbow", 55, 58.2),
            Self::MapleLongbowU => ("Maple Longbow U", 55, 58.3),
            Self::SapphireBolts => ("Sapphire Bolts", 56, 4.7),
            Self::MapleShield => ("Maple Shield", 57, 116.5),
            Self::EmeraldBolts => ("Emerald Bolts", 58, 5.5),
            Self::HuntersSpear => ("Hunters Spear", 60, 9.5),
            Self::AdamantArrow => ("Adamant Arrow", 60, 10.0),
            Self::AdamantBolts => ("Adamant Bolts", 61, 7.0),
            Self::AdamantCrossbow => ("Adamant Crossbow", 61, 41.0),
            Self::MahoganyStock => ("Mahogany Stock", 61, 41.0),
            Self::AdamantCrossbowU => ("Adamant Crossbow U", 61, 82.0),
            Self::AdamantJavelin => ("Adamant Javelin", 62, 10.0),
            Self::RubyBolts => ("Ruby Bolts", 63, 6.3),
            Self::DiamondBolts => ("Diamond Bolts", 65, 7.0),
            Self::YewShortbow => ("Yew Shortbow", 65, 67.5),
            Self::YewShortbowU => ("Yew Shortbow U", 65, 67.5),
            Self::AdamantDart => ("Adamant Dart", 67, 15.0),
            Self::RuniteBolts => ("Runite Bolts", 69, 10.0),
            Self::RuneCrossbow => ("Rune Crossbow", 69, 50.0),
            Self::YewStock => ("Yew Stock", 69, 50.0),
            Self::RuniteCrossbowU => ("Runite CrossbowU", 69, 100.0),
            Self::YewLongbow => ("Yew Longbow", 70, 75.0),
            Self::YewLongbowU => ("Yew Longbow U", 70, 75.0),
            Self::DragonstoneBolts => ("Dragonstone Bolts", 71, 8.2),
            Self::YewShield => ("Yew Shield", 72, 150.0),
            Self::OnyxBolts => ("Onyx Bolts", 73, 9.4),
            Self::RuneArrow => ("Rune Arrow", 75, 12.5),
            Self::AmethystBroadBolts => ("Amethyst Broad Bolts", 76, 10.6),
            Self::RuneJavelin => ("Rune Javelin", 77, 12.4),
            Self::DragonCrossbow => ("Dragon Crossbow", 78, 70.0),
            Self::MagicStock => ("Magic Stock", 78, 70.0),
            Self::ToxicBlowpipe => ("Toxic Blowpipe", 78, 120.0),
            Self::DragonCrossbowU => ("Dragon CrossbowU", 78, 135.0),
            Self::MagicShortbow => ("Magic Shortbow", 80, 83.3),
            Self::MagicShortbowU => ("Magic Shortbow U", 80, 83.3),
            Self::RuneDart => ("Rune Dart", 81, 18.8),
            Self::AmethystArrow => ("Amethyst Arrow", 82, 13.5),
            Self::DragonBolts => ("Dragon Bolts Unf", 84, 12.0),
            Self::AmethystJavelin => ("Amethyst Javelin", 84, 13.5),
            Self::MagicLongbow => ("Magic Longbow", 85, 91.5),
            Self::MagicLongbowU => ("Magic Longbow U", 85, 91.5),
            Self::MagicShield => ("Magic Shield", 87, 183.0),
            Self::DragonArrow => ("Dragon Arrow", 90, 15.0),
            Self::AmethystDart => ("Amethyst Dart", 90, 21.0),
            Self::DragonJavelin => ("Dragon Javelin", 92, 15.0),
            Self::RedwoodShield => ("Redwood Shield", 92, 216.0),
            Self::DragonDart => ("Dragon Dart", 95, 25.0),
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
